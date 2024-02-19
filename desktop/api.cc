#include "api.h"

#include <QDateTime>
#include <QHash>
#include <QJsonDocument>
#include <QJsonObject>
#include <QNetworkRequest>
#include <QRegExp>
#include <QUrl>

Api::Api(QObject *parent) {
    m_activeConnectionList = new ConnectionListModel(this);
    m_historyConnectionList = new ConnectionListModel(this);
    m_networkManager = new QNetworkAccessManager(this);

    m_daemonProcess = new QProcess(this);
    connect(m_daemonProcess, &QProcess::readyReadStandardOutput, this,
            &Api::onDaemonOutput);
    connect(m_daemonProcess, &QProcess::finished, m_daemonProcess,
            &QProcess::deleteLater);
    m_daemonProcess->start("./wsrx", {"daemon"});
    qDebug() << "daemon started";

    m_maintainer = new Maintainer(this, m_activeConnectionList);
    connect(this, &Api::clientChanged, m_maintainer,
            &Maintainer::updateConnectionList);
    connect(m_maintainer, &Maintainer::connectionUnreachable, this,
            &Api::cancelClient);
}

void Api::closeDaemon() {
    m_daemonProcess->kill();
    qDebug() << "daemon killed";
}

void Api::onDaemonOutput() {
    QByteArray buf = m_daemonProcess->readAllStandardOutput();
    QByteArrayList bufList = buf.split('\n');
    for (const auto &line : bufList) {
        qDebug(line);

        // get daemon url
        qsizetype startIdx = line.indexOf("you can access manage api at ");
        if (startIdx != -1) {
            m_daemonUrl = new QString(
                line.mid(startIdx + 29, line.length() - 29 - startIdx - 5));
            // qDebug() << "m_daemonUrl: " << *m_daemonUrl;
        }

        // get tcp server addr
        startIdx = line.indexOf("CREATE tcp server: ");
        if (startIdx != -1) {
            QRegExp rx("CREATE tcp server: (.+):(\\d+) <--wsrx--> (.+)");
            rx.indexIn(line);
            QString wsAddr = rx.cap(3);
            QString tcpAddr = rx.cap(1) + ":" + rx.cap(2);
            QString tcpPort = rx.cap(2);
            QRegExp rx2("ws+://([a-zA-Z0-9.]+):?/?.*");
            rx2.indexIn(wsAddr);
            QString remoteAddr = rx2.cap(1) + "#" + tcpPort;
            m_activeConnectionList->insertData(remoteAddr, wsAddr, tcpAddr);
            emit clientChanged();
        }

        // error report
        startIdx = line.indexOf("failed to bind port");
        if (startIdx != -1) {
            qDebug("daemon failed to bind port");
        }
    }
}

Q_INVOKABLE void Api::launchClient(const QString &bindAddr,
                                   const QString &bindPort,
                                   const QString &targetUrl) {
    if (!targetUrl.startsWith("ws://") && !targetUrl.startsWith("wss://")) {
        qDebug("url is invalid");
        return;
    }
    QJsonObject dataJson;
    dataJson["direction"] = "tcp2ws";
    dataJson["from"] = bindAddr + ":" + bindPort;
    dataJson["to"] = targetUrl;
    QNetworkRequest req = QNetworkRequest(QUrl(*m_daemonUrl + "/pool"));
    req.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");

    m_networkManager->post(req, QJsonDocument(dataJson).toJson());
    qDebug() << "launch client success";
}

Q_INVOKABLE void Api::cancelClient(const QString &remoteAddr,
                                   const QString &wsAddr,
                                   const QString &tcpAddr, const qint32 latency,
                                   const QString &type) {
    if (type == "active") {
        QJsonObject dataJson;
        dataJson["key"] = wsAddr;
        QNetworkRequest req = QNetworkRequest(QUrl(*m_daemonUrl + "/pool"));
        req.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");

        m_networkManager->sendCustomRequest(req, "DELETE",
                                            QJsonDocument(dataJson).toJson());
        m_historyConnectionList->insertData(remoteAddr, wsAddr, tcpAddr,
                                            latency);
        m_activeConnectionList->removeData(remoteAddr);
        qDebug() << "cancel connection [" + remoteAddr + "]";
    } else if (type == "history") {
        m_historyConnectionList->removeData(remoteAddr);
        qDebug() << "remove history connection [" + remoteAddr + "]";
    }
    emit clientChanged();
}

Q_INVOKABLE bool Api::noActiveClients() const {
    return m_activeConnectionList->dataCount() == 0;
}

Q_INVOKABLE bool Api::noHistoryClients() const {
    return m_historyConnectionList->dataCount() == 0;
}

ConnectionListModel *Api::activeConnectionList() const {
    return m_activeConnectionList;
}

ConnectionListModel *Api::historyConnectionList() const {
    return m_historyConnectionList;
}

Api::~Api() = default;