#include "connection_model.h"
#include <QDebug>

ConnectionModel::ConnectionModel(QObject *parent, const QString &remoteAddr, const QString &wsAddr, const QString &tcpAddr, const qint8 latency)
    : QObject(parent) {
        m_wsAddr = wsAddr;
        m_tcpAddr = tcpAddr;
        m_remoteAddr = remoteAddr;
        m_latency = latency;
    }

QString ConnectionModel::websocketAddress() const {
    return m_wsAddr;
}

QString ConnectionModel::tcpAddress() const {
    return m_tcpAddr;
}

QString ConnectionModel::remoteAddress() const {
    return m_remoteAddr;
}

qint8 ConnectionModel::latency() const {
    return m_latency;
}

void ConnectionModel::setLatency(qint8 latency) {
    m_latency = latency;
}

ConnectionModel::~ConnectionModel() = default;