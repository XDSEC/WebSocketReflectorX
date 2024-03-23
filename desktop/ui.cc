#include "ui.h"
#include <QApplication>
#include <QNetworkInterface>
#include <QQmlComponent>
#include <QQmlContext>
#include <QQmlEngine>
#include <QQuickWindow>
#include <QTranslator>
#include "daemon.h"
#include "log.h"
#include "pool.h"

Ui::Ui(QObject *parent) : QObject(parent) {
    m_uiEngine = new QQmlEngine(this);
    m_translator = new QTranslator(this);
    auto ok =
        m_translator->load(QString(":/resources/i18n/%1.qm").arg("zh_CN"));
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
    m_window = qobject_cast<QQuickWindow *>(m_uiComponent->create());
}

Ui::~Ui() = default;

void Ui::requestToQuit() {
    m_window->close();
    m_window->deleteLater();
    m_uiComponent->deleteLater();
    m_uiEngine->deleteLater();
    QApplication::exit(0);
}

void Ui::show() {
    if (m_uiComponent->isError())
        qWarning() << m_uiComponent->errors();
    m_window->show();
}
