#pragma once

#include <QObject>
#include <QAbstractListModel>

enum EventLevel { INFO, WARNING, ERROR, SUCCESS };

class Log : public QObject {
    Q_GADGET
    Q_PROPERTY(QString timestamp READ timestamp WRITE setTimestamp)
    Q_PROPERTY(EventLevel level READ level WRITE setLevel)
    Q_PROPERTY(QString message READ message WRITE setMessage)
    Q_PROPERTY(QString target READ target WRITE setTarget)
   private:
    QString m_timestamp;
    EventLevel m_level;
    QString m_message;
    QString m_target;

   public:
    Log(const QString &timestamp, EventLevel level, const QString &message,
        const QString &target);

    Log(const Log &other);

    Log();

    ~Log() override;

    Log &operator=(const Log &other);

    static Log fromJson(const QString &json);

    [[nodiscard]] QString timestamp() const;

    void setTimestamp(const QString &timestamp);

    [[nodiscard]] EventLevel level() const;

    [[nodiscard]] QString levelString() const;

    void setLevel(EventLevel level);

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

    void appendLog(const Log &log);

    QVector<Log> *logs() const;
};
