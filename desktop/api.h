#pragma once

#include "model/connection_list_model.h"
#include "maintainer.h"
#include <QNetworkAccessManager>
#include <QObject>
#include <QProcess>

class Api : public QObject {
    Q_OBJECT
    Q_PROPERTY(bool noActiveClient READ noActiveClients NOTIFY clientChanged);
    Q_PROPERTY(bool noHistoryClient READ noHistoryClients NOTIFY clientChanged);

  private:
    QProcess *m_daemonProcess;
    Maintainer *m_maintainer;
    QString *m_daemonUrl;
    QNetworkAccessManager *m_networkManager;
    ConnectionListModel *m_activeConnectionList;
    ConnectionListModel *m_historyConnectionList;

  public:
    explicit Api(QObject *parent = nullptr);
    ~Api() override;
    void closeDaemon();
    ConnectionListModel *activeConnectionList() const;
    ConnectionListModel *historyConnectionList() const;
  signals:
    void clientChanged();
  public slots:
    Q_INVOKABLE bool noActiveClients() const;
    Q_INVOKABLE bool noHistoryClients() const;
    Q_INVOKABLE void launchClient(const QString &bindAddr,
                                  const QString &bindPort,
                                  const QString &targetUrl);
    Q_INVOKABLE void cancelClient(const QString &remoteAddr,
                                  const QString &wsAddr, const QString &tcpAddr,
                                  const qint8 latency, const QString &type);
  private slots:
    void onDaemonOutput();
};
