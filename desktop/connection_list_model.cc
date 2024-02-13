#include "connection_list_model.h"

ConnectionListModel::ConnectionListModel(QObject *parent)
    : QAbstractListModel(parent){
        m_connectionList = new QList<ConnectionModel *>();
    }

int ConnectionListModel::rowCount(const QModelIndex &parent) const {
    return m_connectionList->size();
}

QVariant ConnectionListModel::data(const QModelIndex &index, int role) const {
    if (index.row() < m_connectionList->size()) {
        switch (role) {
            case WebsocketAddressRole:
                return m_connectionList->at(index.row())->websocketAddress();
            case TcpAddressRole:
                return m_connectionList->at(index.row())->tcpAddress();
            case RemoteAddressRole:
                return m_connectionList->at(index.row())->remoteAddress();
            case LagencyRole:
                return m_connectionList->at(index.row())->lagency();
            default:
                return QVariant();
        }
    }
    return QVariant();
}

QHash<int, QByteArray> ConnectionListModel::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[WebsocketAddressRole] = "websocketAddress";
    roles[TcpAddressRole] = "tcpAddress";
    roles[RemoteAddressRole] = "remoteAddress";
    roles[LagencyRole] = "lagency";
    return roles;
}

void ConnectionListModel::insertData(const QString &remoteAddr, const QString &wsAddr, const QString &tcpAddr) {
    for (int i = 0; i < m_connectionList->size(); i++) {
        if (m_connectionList->at(i)->remoteAddress() == remoteAddr) {
            return;
        }
    }
    beginInsertRows(QModelIndex(), m_connectionList->size(), m_connectionList->size());
    m_connectionList->append(new ConnectionModel(this, remoteAddr, wsAddr, tcpAddr));
    endInsertRows();
}

void ConnectionListModel::removeData(const QString &remoteAddr) {
    for (int i = 0; i < m_connectionList->size(); i++) {
        if (m_connectionList->at(i)->remoteAddress() == remoteAddr) {
            beginRemoveRows(QModelIndex(), i, i);
            m_connectionList->removeAt(i);
            endRemoveRows();
            return;
        }
    }
}

int ConnectionListModel::dataCount() const {
    return m_connectionList->size();
}