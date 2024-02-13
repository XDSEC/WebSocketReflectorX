#pragma once

#include <QObject>
#include <QProcess>
#include <QNetworkAccessManager>
#include "connection_list_model.h"

class Api : public QObject {
Q_OBJECT
Q_PROPERTY(bool noActiveClient READ noActiveClients NOTIFY clientChanged);
Q_PROPERTY(bool noHistoryClient READ noHistoryClients NOTIFY clientChanged);
private:
    QProcess *m_daemonProcess;
    QString *m_daemonUrl;
    QNetworkAccessManager *m_manager;
    ConnectionListModel *m_activeConnectionList;
    ConnectionListModel *m_historyConnectionList;
public:
    explicit Api(QObject* parent = nullptr);
    ~Api() override;
    ConnectionListModel *activeConnectionList() const;
    ConnectionListModel *historyConnectionList() const;
    Q_INVOKABLE bool noActiveClients() const;
    Q_INVOKABLE bool noHistoryClients() const;
    Q_INVOKABLE void launchClient(const QString &bindAddr, const QString &bindPort, const QString &targetUrl);
    Q_INVOKABLE void cancelClient(const QString &remoteAddr, const QString &wsAddr, const QString &tcpAddr, const QString &type);
    void closeDaemon();
signals:
    void clientChanged();
private slots:
    void onDaemonOutput();
};


