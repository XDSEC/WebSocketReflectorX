#pragma once

#include "model/connection_list_model.h"
#include <QNetworkReply>
#include <QObject>
#include <QTimer>

class Maintainer : public QObject {
    Q_OBJECT
  public:
    explicit Maintainer(QObject *parent = nullptr,
                        ConnectionListModel *connectionList = nullptr);
    ~Maintainer() override;
    void sendConnectionUnreachable(const QString &remoteAddr);

  signals:
    void connectionUnreachable(const QString &remoteAddr, const QString &wsAddr,
                               const QString &tcpAddr, const qint8 latency,
                               const QString &type = "active");
  public slots:
    void updateConnectionList();
  private slots:
    void pingConnections();
    void onNetworkReply(QNetworkReply *reply);

  private:
    const uint MAX_RETRY = 3;
    QTimer *m_timer;
    ConnectionListModel *m_maintainModel;
    QNetworkAccessManager *m_networkManager;
    QList<ConnectionModel *> *m_connectionList;
    QHash<QString, quint64> *m_pingTimestamps;
    QHash<QString, qint8> *m_failedRecords;
};