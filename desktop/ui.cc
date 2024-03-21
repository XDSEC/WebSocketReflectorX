#include "ui.h"
#include <QApplication>
#include <QNetworkInterface>
#include <QQmlComponent>
#include <QQmlContext>
#include <QQmlEngine>
#include <QQuickWindow>
#include <QTranslator>
#include "daemon.h"

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
    m_uiEngine->retranslate();
    m_uiComponent = new QQmlComponent(m_uiEngine, this);
    m_uiComponent->loadUrl(QUrl(u"qrc:/ui/Main.qml"_qs));
    m_window = qobject_cast<QQuickWindow *>(m_uiComponent->create());
}

Ui::~Ui() = default;

quint8 Ui::page() const { return m_page; }

void Ui::setPage(quint8 page) {
    if (m_page == page)
        return;
    m_page = page;
    emit pageChanged(page);
}


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
