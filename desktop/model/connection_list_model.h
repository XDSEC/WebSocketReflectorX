#pragma once

#include "connection_model.h"
#include <QAbstractListModel>
#include <QObject>

class ConnectionListModel : public QAbstractListModel {
    Q_OBJECT
  public:
    enum ConnectionListRoles {
        WebsocketAddressRole = Qt::UserRole + 1,
        TcpAddressRole,
        RemoteAddressRole,
        LatencyRole
    };
    explicit ConnectionListModel(QObject *parent = nullptr);

    [[nodiscard]] int rowCount(const QModelIndex &parent) const override;
    [[nodiscard]] QVariant data(const QModelIndex &index,
                                int role) const override;
    [[nodiscard]] QHash<int, QByteArray> roleNames() const override;
    [[nodiscard]] bool setData(const QModelIndex &index, const QVariant &value,
                               int role) override;

    void insertData(const QString &remoteAddr, const QString &wsAddr,
                    const QString &tcpAddr, const qint8 latency = -1);
    void removeData(const QString &remoteAddr);

    int dataCount() const;
    QList<ConnectionModel *> *connectionList() const;
    bool updateLatency(const QString &remoteAddr, qint8 latency);

  private:
    QList<ConnectionModel *> *m_connectionList;
};