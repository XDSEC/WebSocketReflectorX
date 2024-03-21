#pragma once

#include <QObject>

class Daemon : public QObject {
    Q_OBJECT
    Q_PROPERTY(QStringList availableAddresses READ availableAddresses WRITE
                   setAvailableAddresses NOTIFY availableAddressesChanged)
    Q_PROPERTY(QString systemInfo READ systemInfo NOTIFY systemInfoChanged)

   private:
    QStringList m_availableAddresses{"127.0.0.1", "0.0.0.0"};

   public:
    explicit Daemon(QObject *parent = nullptr);

    ~Daemon() override;

    [[nodiscard]] QStringList availableAddresses() const;
    
    void setAvailableAddresses(const QStringList &availableAddresses);

    [[nodiscard]] QString systemInfo() const;

   public slots:

    Q_INVOKABLE void refreshAvailableAddresses();

   signals:

    void availableAddressesChanged(const QStringList &availableAddresses);
    void systemInfoChanged(const QString &systemInfo);
};
