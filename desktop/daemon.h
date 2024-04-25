#pragma once

#include <QAbstractListModel>
#include <QObject>

class QProcess;
class QNetworkAccessManager;
class QTimer;
class LogList;
class LinkList;

class Daemon : public QObject {
    Q_OBJECT
    Q_PROPERTY(QStringList availableAddresses READ availableAddresses WRITE setAvailableAddresses NOTIFY
                   availableAddressesChanged)
    Q_PROPERTY(QString systemInfo READ systemInfo NOTIFY systemInfoChanged)

  private:
    QStringList m_availableAddresses{"127.0.0.1", "0.0.0.0"};
    QProcess* m_daemon;
    QString m_apiRoot = "http://127.0.0.1:3307/";
    LogList* m_logs;
    LinkList* m_links;
    QNetworkAccessManager* m_network;
    QTimer* m_refreshTimer;
    QTimer* m_heartbeatTimer;

    void syncPool();

    void heartbeat();

    void checkOrigins();

  public:
    explicit Daemon(QObject* parent = nullptr);

    ~Daemon() override;

    [[nodiscard]] QStringList availableAddresses() const;

    void setAvailableAddresses(const QStringList& availableAddresses);

    [[nodiscard]] QString systemInfo() const;

    LogList* logs() const;

    LinkList* links() const;

  public slots:
    Q_INVOKABLE void exportLogs(const QUrl& path) const;

    Q_INVOKABLE void refreshAvailableAddresses();

    Q_INVOKABLE void requestConnect(const QString& address, const QString& host, const quint16 port);

    Q_INVOKABLE void requestDisconnect(const QString& local_address);

  signals:
    void availableAddressesChanged(const QStringList& availableAddresses);

    void systemInfoChanged(const QString& systemInfo);

    void connected(bool success, const QString& message);

    void disconnected(bool success, const QString& message);
};
