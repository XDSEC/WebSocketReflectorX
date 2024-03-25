#include "ui.h"

#include <QApplication>
#include <QLocale>
#include <QMutex>
#include <QMutexLocker>
#include <QNetworkInterface>
#include <QQmlComponent>
#include <QQmlContext>
#include <QQmlEngine>
#include <QQuickWindow>
#include <QSettings>
#include <QTranslator>
#ifdef Q_OS_UNIX
#include <QSocketNotifier>
#include <signal.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>
#endif
#include "daemon.h"
#include "log.h"
#include "pool.h"

Ui* Ui::m_instance = nullptr;

#ifdef Q_OS_UNIX
int Ui::sighupFd[2] = {};
int Ui::sigtermFd[2] = {};
int Ui::sigsegvFd[2] = {};
int Ui::sigintFd[2] = {};
#endif

Ui* Ui::instance(QObject* parent) {
    static QMutex mutex;
    if (m_instance == nullptr) {
        QMutexLocker locker(&mutex);
        if (m_instance == nullptr) m_instance = new Ui(parent);
        locker.unlock();
    }
    return m_instance;
}

Ui::Ui(QObject* parent) : QObject(parent) {
#ifdef Q_OS_UNIX
    if (::socketpair(AF_UNIX, SOCK_STREAM, 0, sighupFd)) qFatal("Couldn't create HUP socketpair");
    if (::socketpair(AF_UNIX, SOCK_STREAM, 0, sigtermFd)) qFatal("Couldn't create TERM socketpair");
    if (::socketpair(AF_UNIX, SOCK_STREAM, 0, sigsegvFd)) qFatal("Couldn't create SEGV socketpair");
    if (::socketpair(AF_UNIX, SOCK_STREAM, 0, sigintFd)) qFatal("Couldn't create INT socketpair");
    snHup = new QSocketNotifier(sighupFd[1], QSocketNotifier::Read, this);
    connect(snHup, SIGNAL(activated(QSocketDescriptor)), this, SLOT(sighupHandler()));
    snTerm = new QSocketNotifier(sigtermFd[1], QSocketNotifier::Read, this);
    connect(snTerm, SIGNAL(activated(QSocketDescriptor)), this, SLOT(sigtermHandler()));
    snSegv = new QSocketNotifier(sigsegvFd[1], QSocketNotifier::Read, this);
    connect(snSegv, SIGNAL(activated(QSocketDescriptor)), this, SLOT(sigsegvHandler()));
    snInt = new QSocketNotifier(sigintFd[1], QSocketNotifier::Read, this);
    connect(snInt, SIGNAL(activated(QSocketDescriptor)), this, SLOT(sigintHandler()));
#endif
    loadSettings();
    m_uiEngine = new QQmlEngine(this);
    m_translator = new QTranslator(this);
    auto ok = m_translator->load(QString(":/resources/i18n/%1.qm").arg(m_language));
    if (!ok) {
        qWarning() << "failed to load translator";
    }
    QApplication::installTranslator(m_translator);
    m_daemon = new Daemon(this);
    m_uiEngine->rootContext()->setContextProperty("ui", this);
    m_uiEngine->rootContext()->setContextProperty("daemon", m_daemon);
    m_uiEngine->rootContext()->setContextProperty("logs", m_daemon->logs());
    m_uiEngine->rootContext()->setContextProperty("links", m_daemon->links());
    m_uiEngine->retranslate();
    m_uiComponent = new QQmlComponent(m_uiEngine, this);
    m_uiComponent->loadUrl(QUrl(u"qrc:/ui/Main.qml"_qs));
    m_window = qobject_cast<QQuickWindow*>(m_uiComponent->create());
}

Ui::~Ui() = default;

void Ui::loadSettings() {
    auto locale = QLocale::system();
    QSettings settings;
    settings.beginGroup("ui");
    setRunningInTray(settings.value("runningInTray", false).toBool());
    setIsDark(settings.value("isDark", true).toBool());
    m_language = settings.value("language", locale.name()).toString();
    settings.endGroup();
}

void Ui::saveSettings() {
    QSettings settings;
    settings.beginGroup("ui");
    settings.setValue("runningInTray", runningInTray());
    settings.setValue("isDark", isDark());
    settings.setValue("language", language());
    settings.endGroup();
}

void Ui::requestToQuit() {
    m_window->close();
    m_window->deleteLater();
    m_uiComponent->deleteLater();
    m_uiEngine->deleteLater();
    m_daemon->deleteLater();
    saveSettings();
    QApplication::exit(0);
}

void Ui::show() {
    if (m_uiComponent->isError()) qWarning() << m_uiComponent->errors();
    m_window->show();
}

bool Ui::runningInTray() const { return m_runningInTray; }

void Ui::setRunningInTray(bool runningInTray) {
    // qDebug() << runningInTray;
    if (m_runningInTray == runningInTray) return;
    m_runningInTray = runningInTray;
    emit runningInTrayChanged(runningInTray);
}

bool Ui::isDark() const { return m_isDark; }

void Ui::setIsDark(bool isDark) {
    if (m_isDark == isDark) return;
    m_isDark = isDark;
    emit isDarkChanged(isDark);
}

QString Ui::language() const { return m_language; }

void Ui::setLanguage(const QString& language) {
    // qDebug() << language;
    if (m_language == language) return;
    QApplication::removeTranslator(m_translator);
    auto ok = m_translator->load(QString(":/resources/i18n/%1.qm").arg(language));
    if (!ok) {
        qWarning() << "failed to load translator";
        Q_UNUSED(m_translator->load(QString(":/resources/i18n/%1.qm").arg(m_language)));
        QApplication::installTranslator(m_translator);
        m_uiEngine->retranslate();
        return;
    }
    QApplication::installTranslator(m_translator);
    m_uiEngine->retranslate();
    m_language = language;
    emit languageChanged(language);
}

#ifdef Q_OS_UNIX

void Ui::sigtermHandler() {
    snTerm->setEnabled(false);
    char tmp;
    ::read(sigtermFd[1], &tmp, sizeof(tmp));
    qDebug() << "SIGTERM received";
    instance()->requestToQuit();
    QApplication::exit(0);
    snTerm->setEnabled(true);
}

void Ui::sighupHandler() {
    snHup->setEnabled(false);
    char tmp;
    ::read(sighupFd[1], &tmp, sizeof(tmp));
    qDebug() << "SIGHUP received";
    snHup->setEnabled(true);
}

void Ui::sigsegvHandler() {
    snSegv->setEnabled(false);
    char tmp;
    ::read(sigsegvFd[1], &tmp, sizeof(tmp));
    qDebug() << "SIGSEGV received";
    instance()->requestToQuit();
    QApplication::exit(139);
    snSegv->setEnabled(true);
}

void Ui::sigintHandler() {
    snInt->setEnabled(false);
    char tmp;
    ::read(sigintFd[1], &tmp, sizeof(tmp));
    qDebug() << "SIGINT received";
    instance()->requestToQuit();
    QApplication::exit(130);
    snInt->setEnabled(true);
}

void Ui::sigtermSigHandler(int) {
    char a = 1;
    ::write(sigtermFd[0], &a, sizeof(a));
}

void Ui::sighupSigHandler(int) {
    char a = 1;
    ::write(sighupFd[0], &a, sizeof(a));
}

void Ui::sigsegvSigHandler(int) {
    char a = 1;
    ::write(sigsegvFd[0], &a, sizeof(a));
}

void Ui::sigintSigHandler(int) {
    char a = 1;
    ::write(sigintFd[0], &a, sizeof(a));
}

#endif