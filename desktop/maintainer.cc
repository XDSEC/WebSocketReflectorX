#include "maintainer.h"
#include <QNetworkAccessManager>

Maintainer::Maintainer(QObject *parent,
                       ConnectionListModel *activeConnectionList)
    : QObject(parent) {
    m_maintainModel = activeConnectionList;
    m_pingTimestamps = new QHash<QString, quint64>();
    m_failedRecords = new QHash<QString, qint8>();

    m_timer = new QTimer(this);
    m_timer->setInterval(3000);
    connect(m_timer, &QTimer::timeout, this, &Maintainer::pingConnections);

    m_networkManager = new QNetworkAccessManager(this);
    m_networkManager->setTransferTimeout(2000);
    connect(m_networkManager, &QNetworkAccessManager::finished, this,
            &Maintainer::onNetworkReply);
}

void Maintainer::updateConnectionList() {
    m_connectionList = m_maintainModel->connectionList();
    m_timer->start();
}

void Maintainer::pingConnections() {
    uint cnt = m_connectionList->size();
    for (uint i = 0; i < cnt; i++) {
        ConnectionModel *connection = m_connectionList->at(i);
        QUrl url = QUrl(connection->websocketAddress());
        if (url.scheme() == "wss") {
            url.setScheme("https");
        } else if (url.scheme() == "ws") {
            url.setScheme("http");
        } else {
            qDebug() << "invalid scheme: " << url.scheme();
        }
        QString remoteAddr = connection->remoteAddress();
        QNetworkRequest req = QNetworkRequest(url);
        req.setRawHeader("remote-addr", remoteAddr.toUtf8());
        quint64 start = QDateTime::currentMSecsSinceEpoch();
        m_pingTimestamps->insert(remoteAddr, start);
        m_networkManager->sendCustomRequest(req, "OPTIONS");
    }
}

void Maintainer::onNetworkReply(QNetworkReply *reply) {
    quint64 end = QDateTime::currentMSecsSinceEpoch();
    QString remoteAddr = reply->request().rawHeader("remote-addr");
    quint64 start = m_pingTimestamps->value(remoteAddr);
    m_pingTimestamps->remove(remoteAddr);

    if (reply->error() != QNetworkReply::NoError) {
        qWarning() << "ping reply error: " << reply->errorString();
        qint8 failedCnt = m_failedRecords->value(remoteAddr, 0);
        m_failedRecords->insert(remoteAddr, failedCnt + 1);
        if (failedCnt > MAX_RETRY) {
            sendConnectionUnreachable(remoteAddr);
        }
        return;
    }
    qint8 latency = end - start;
    if (!m_maintainModel->updateLatency(remoteAddr, latency)) {
        qWarning() << "failed to update latency";
    }

    if (m_failedRecords->contains(remoteAddr)) {
        m_failedRecords->remove(remoteAddr);
    }

    reply->deleteLater();
}

void Maintainer::sendConnectionUnreachable(const QString &remoteAddr) {
    auto size = m_connectionList->size();
    for (int i = 0; i < size; i++) {
        ConnectionModel *connection = m_connectionList->at(i);
        if (connection->remoteAddress() == remoteAddr) {
            QString wsAddr = connection->websocketAddress();
            QString tcpAddr = connection->tcpAddress();
            qint8 latency = connection->latency();
            emit connectionUnreachable(remoteAddr, wsAddr, tcpAddr, latency);
            return;
        }
    }
}

Maintainer::~Maintainer() = default;
