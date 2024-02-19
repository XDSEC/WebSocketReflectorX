#include "ui.h"
#include <QApplication>
#include <QNetworkInterface>
#include <QQmlComponent>
#include <QQmlContext>
#include <QQmlEngine>
#include <QQuickWindow>
#include <QTranslator>

Ui::Ui(QObject *parent) {
    m_uiEngine = new QQmlEngine(this);
    m_translator = new QTranslator(this);
    m_api = new Api(this);
    auto ok =
        m_translator->load(QString(":/resources/i18n/%1.qm").arg("zh_CN"));
    if (!ok) {
        qWarning() << "failed to load translator";
    }
    QApplication::installTranslator(m_translator);
    m_uiEngine->rootContext()->setContextProperty("ui", this);
    m_uiEngine->rootContext()->setContextProperty("api", m_api);
    m_uiEngine->rootContext()->setContextProperty(
        "activeConnectionList", m_api->activeConnectionList());
    m_uiEngine->rootContext()->setContextProperty(
        "historyConnectionList", m_api->historyConnectionList());
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

QStringList Ui::availableAddresses() const { return m_availableAddresses; }

void Ui::setAvailableAddresses(const QStringList &availableAddresses) {
    if (m_availableAddresses == availableAddresses)
        return;
    m_availableAddresses = availableAddresses;
    emit availableAddressesChanged(availableAddresses);
}

Q_INVOKABLE void Ui::refreshAvailableAddresses() {
    auto addresses = QNetworkInterface::allAddresses();
    QStringList availableAddresses;
    availableAddresses.append("127.0.0.1");
    for (const auto &address : addresses) {
        if (address.isLoopback())
            continue;
        if (address.protocol() != QAbstractSocket::IPv4Protocol)
            continue;
        availableAddresses.append(address.toString());
    }
    availableAddresses.append("0.0.0.0");
    setAvailableAddresses(availableAddresses);
}

void Ui::requestToQuit() {
    m_window->close();
    m_window->deleteLater();
    m_uiComponent->deleteLater();
    m_uiEngine->deleteLater();
    m_api->closeDaemon();
    QApplication::exit(0);
}

void Ui::show() {
    if (m_uiComponent->isError())
        qWarning() << m_uiComponent->errors();
    m_window->show();
}
