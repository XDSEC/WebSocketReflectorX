#pragma once

#include <QAbstractListModel>
#include <QObject>

class QProcess;
class QNetworkAccessManager;
class QTimer;
class LogList;
class LinkList;
class QUrl;

class Daemon : public QObject {
    Q_OBJECT
    Q_PROPERTY(QStringList availableAddresses READ availableAddresses WRITE setAvailableAddresses NOTIFY
                   availableAddressesChanged)
    Q_PROPERTY(QString systemInfo READ systemInfo NOTIFY systemInfoChanged)
    Q_PROPERTY(quint16 apiPort READ apiPort NOTIFY apiPortChanged)

  private:
    QStringList m_availableAddresses{"127.0.0.1", "0.0.0.0"};
    QProcess* m_daemon;
    quint16 m_apiPort = 3307;
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

    [[nodiscard]] quint16 apiPort() const;

    void setApiPort(quint16 p);

    LogList* logs() const;

    LinkList* links() const;

    QString apiRoot() const;

    QUrl service(const QString& name) const;

    void launch();

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

    void apiPortChanged(quint16 n);
};
