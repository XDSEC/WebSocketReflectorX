#pragma once

#include <QAbstractListModel>
#include <QObject>

class QProcess;

enum LogLevel { INFO, WARNING, ERROR, SUCCESS };

class Log : public QObject {
    Q_GADGET
    Q_PROPERTY(QString timestamp READ timestamp WRITE setTimestamp)
    Q_PROPERTY(LogLevel level READ level WRITE setLevel)
    Q_PROPERTY(QString message READ message WRITE setMessage)
    Q_PROPERTY(QString target READ target WRITE setTarget)
   private:
    QString m_timestamp;
    LogLevel m_level;
    QString m_message;
    QString m_target;

   public:
    Log(const QString &timestamp, LogLevel level, const QString &message, const QString &target);

    Log(const Log &other);

    Log();

    ~Log() override;

    Log& operator=(const Log &other);

    static Log fromJson(const QString &json);

    [[nodiscard]] QString timestamp() const;

    void setTimestamp(const QString &timestamp);

    [[nodiscard]] LogLevel level() const;

    void setLevel(LogLevel level);

    [[nodiscard]] QString message() const;

    void setMessage(const QString &message);

    [[nodiscard]] QString target() const;

    void setTarget(const QString &target);
};

Q_DECLARE_METATYPE(Log)

class LogList : public QAbstractListModel {
    Q_OBJECT

   private:
    QVector<Log> m_list{};

   public:

    enum LogListRoles {
        LevelRole = Qt::UserRole + 1,
        TimestampRole,
        MessageRole,
        TargetRole
    };

    explicit LogList(QObject *parent = nullptr);

    ~LogList() override;

    [[nodiscard]] int rowCount(const QModelIndex &parent) const override;

    [[nodiscard]] QVariant data(const QModelIndex &index,
                                int role) const override;

    [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

    void appendLogs(const QString &json);
};

class Daemon : public QObject {
    Q_OBJECT
    Q_PROPERTY(QStringList availableAddresses READ availableAddresses WRITE
                   setAvailableAddresses NOTIFY availableAddressesChanged)
    Q_PROPERTY(QString systemInfo READ systemInfo NOTIFY systemInfoChanged)

   private:
    QStringList m_availableAddresses{"127.0.0.1", "0.0.0.0"};
    QProcess *m_daemon;
    QString m_api_root = "http://127.0.0.1:3307/";
    LogList *m_logs;

   public:
    explicit Daemon(QObject *parent = nullptr);

    ~Daemon() override;

    [[nodiscard]] QStringList availableAddresses() const;

    void setAvailableAddresses(const QStringList &availableAddresses);

    [[nodiscard]] QString systemInfo() const;

    LogList* logs() const;

   public slots:

    Q_INVOKABLE void refreshAvailableAddresses();

   signals:

    void availableAddressesChanged(const QStringList &availableAddresses);
    void systemInfoChanged(const QString &systemInfo);
};
