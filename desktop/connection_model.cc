#include "connection_model.h"

ConnectionModel::ConnectionModel(QObject *parent, const QString &remoteAddr, const QString &wsAddr, const QString &tcpAddr)
    : QObject(parent) {
        m_wsAddr = wsAddr;
        m_tcpAddr = tcpAddr;
        m_remoteAddr = remoteAddr;
        m_lagency = -1;
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

uint ConnectionModel::lagency() const {
    return m_lagency;
}

ConnectionModel::~ConnectionModel() = default;