#include "ui.h"
#include "variables.h"

#include <QApplication>
#include <QLocale>
#include <QMutex>
#include <QMutexLocker>
#include <QNetworkInterface>
#include <QNetworkAccessManager>
#include <QNetworkReply>
#include <QJsonDocument>
#include <QJsonObject>
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
#include "cors.h"
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
#ifdef Q_OS_MACOS
    setIsMac(true);
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
    m_websites = new WebsiteList(this, m_daemon->apiPort());
    m_uiEngine->rootContext()->setContextProperty("ui", this);
    m_uiEngine->rootContext()->setContextProperty("daemon", m_daemon);
    m_uiEngine->rootContext()->setContextProperty("logs", m_daemon->logs());
    m_uiEngine->rootContext()->setContextProperty("links", m_daemon->links());
    m_uiEngine->rootContext()->setContextProperty("websites", m_websites);
    m_uiEngine->retranslate();
    m_uiComponent = new QQmlComponent(m_uiEngine, this);
    m_uiComponent->loadUrl(QUrl(u"qrc:/ui/Main.qml"_qs));
    m_window = qobject_cast<QQuickWindow*>(m_uiComponent->create());
    m_networkManager = new QNetworkAccessManager(this);
    setNewVersion("");
    setHasNewVersion(false);
    setUpdateUrl("");
    // setUpdateUrl("https://github.com//XDSEC/WebSocketReflectorX/releases/latest");
    setVersion(VERSION);
    checkUpdates();
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

Q_INVOKABLE void Ui::onSecondaryInstanceMessageReceived(quint32 instanceId, const QByteArray& message) {
    const QString link = message;
    m_daemon->requestConnect(link, "127.0.0.1", 0);
}

Q_INVOKABLE void Ui::onSecondaryInstanceStarted() {
    m_window->show();
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

bool Ui::isMac() const { return m_isMac; }

void Ui::setIsMac(bool isMac) {
    if (m_isMac == isMac) return;
    m_isMac = isMac;
    emit isMacChanged(isMac);
}

QString Ui::version() const { return m_version; }

void Ui::setVersion(const QString& version) {
    if (m_version == version) return;
    m_version = version;
    emit versionChanged(version);
}

bool Ui::hasNewVersion() const { return m_hasNewVersion; }

void Ui::setHasNewVersion(bool hasNewVersion) {
    if (m_hasNewVersion == hasNewVersion) return;
    m_hasNewVersion = hasNewVersion;
    emit hasNewVersionChanged(hasNewVersion);
}

QString Ui::newVersion() const { return m_newVersion; }

void Ui::setNewVersion(const QString& newVersion) {
    if (m_newVersion == newVersion) return;
    m_newVersion = newVersion;
    emit newVersionChanged(newVersion);
}

QString Ui::updateUrl() const { return m_updateUrl; }

void Ui::setUpdateUrl(const QString& updateUrl) {
    if (m_updateUrl == updateUrl) return;
    m_updateUrl = updateUrl;
    emit updateUrlChanged(updateUrl);
}

void Ui::checkUpdates() {
    auto url = QUrl("https://api.github.com/repos/XDSEC/WebSocketReflectorX/releases/latest");
    auto request = QNetworkRequest(url);
    request.setRawHeader("Accept", "application/vnd.github+json");
    request.setRawHeader("X-GitHub-Api-Version", "2022-11-28");
    auto reply = m_networkManager->get(request);
    connect(reply, &QNetworkReply::finished, this, [=]() {
        if (reply->error() != QNetworkReply::NoError) {
            qWarning() << reply->errorString();
            return;
        }
        auto data = reply->readAll();
        auto json = QJsonDocument::fromJson(data).object();
        auto version = json["tag_name"].toString();
        auto current = QString(VERSION);
        if (version > current) {
            setHasNewVersion(true);
            setNewVersion(version);
            setUpdateUrl(json["html_url"].toString());
        } else {
            setHasNewVersion(false);
            setNewVersion("");
            setUpdateUrl("");
        }
    });
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
