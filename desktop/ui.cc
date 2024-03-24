#include "ui.h"

#include <QApplication>
#include <QLocale>
#include <QNetworkInterface>
#include <QQmlComponent>
#include <QQmlContext>
#include <QQmlEngine>
#include <QQuickWindow>
#include <QSettings>
#include <QTranslator>

#include "daemon.h"
#include "log.h"
#include "pool.h"

Ui::Ui(QObject* parent) : QObject(parent) {
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
    saveSettings();
    QApplication::exit(0);
}

void Ui::show() {
    if (m_uiComponent->isError()) qWarning() << m_uiComponent->errors();
    m_window->show();
}

bool Ui::runningInTray() const { return m_runningInTray; }

void Ui::setRunningInTray(bool runningInTray) {
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
