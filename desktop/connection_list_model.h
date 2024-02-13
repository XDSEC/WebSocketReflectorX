#pragma once

#include <QAbstractListModel>
#include <QObject>
#include "connection_model.h"

class ConnectionListModel : public QAbstractListModel {
    Q_OBJECT
    public:
    enum ConnectionListRoles {
        WebsocketAddressRole = Qt::UserRole + 1,
        TcpAddressRole,
        RemoteAddressRole,
        LagencyRole
    };
    explicit ConnectionListModel(QObject *parent = nullptr);

    [[nodiscard]] int rowCount(const QModelIndex &parent) const override;

    [[nodiscard]] QVariant data(const QModelIndex &index,
                                int role) const override;

    [[nodiscard]] QHash<int, QByteArray> roleNames() const override;
    
    void insertData(const QString &remoteAddr, const QString &wsAddr, const QString &tcpAddr);
    void removeData(const QString &remoteAddr);
    int dataCount() const;
    private:
    QList<ConnectionModel *> *m_connectionList;
};