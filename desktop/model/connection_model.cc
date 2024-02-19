#include "connection_model.h"

ConnectionModel::ConnectionModel(QObject *parent, const QString &remoteAddr,
                                 const QString &wsAddr, const QString &tcpAddr,
                                 const qint32 latency)
    : QObject(parent) {
    m_wsAddr = wsAddr;
    m_tcpAddr = tcpAddr;
    m_remoteAddr = remoteAddr;
    m_latency = latency;
}

QString ConnectionModel::websocketAddress() const { return m_wsAddr; }

QString ConnectionModel::tcpAddress() const { return m_tcpAddr; }

QString ConnectionModel::remoteAddress() const { return m_remoteAddr; }

qint32 ConnectionModel::latency() const { return m_latency; }

void ConnectionModel::setLatency(qint32 latency) { m_latency = latency; }

ConnectionModel::~ConnectionModel() = default;