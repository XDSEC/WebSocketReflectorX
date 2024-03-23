#include "log.h"

#include <QDateTime>
#include <QJsonDocument>
#include <QJsonObject>

Log::Log(const QString &timestamp, EventLevel level, const QString &message, const QString &target)
    : m_timestamp(timestamp), m_level(level), m_message(message), m_target(target) {}

Log::Log(const Log &other) {
    m_timestamp = other.m_timestamp;
    m_level = other.m_level;
    m_message = other.m_message;
    m_target = other.m_target;
}

Log::Log() {
    m_timestamp = QDateTime::currentDateTime().toString(Qt::ISODate);
    m_level = EventLevel::INFO;
    m_message = "EMPTY";
    m_target = "EMPTY";
}

Log::~Log() = default;

Log &Log::operator=(const Log &other) {
    if (this == &other) return *this;
    m_timestamp = other.m_timestamp;
    m_level = other.m_level;
    m_message = other.m_message;
    return *this;
}

Log Log::fromJson(const QString &json) {
    auto doc = QJsonDocument::fromJson(json.toUtf8());
    auto obj = doc.object();
    auto timestamp = obj["timestamp"].toString();
    QString level;
    if (obj.contains("level"))
        level = obj["level"].toString();
    else
        level = "INFO";
    EventLevel level_enum;
    if (level == "INFO")
        level_enum = EventLevel::INFO;
    else if (level == "WARN")
        level_enum = EventLevel::WARNING;
    else if (level == "ERROR")
        level_enum = EventLevel::ERROR;
    else
        level_enum = EventLevel::INFO;
    auto message = obj["fields"].toObject()["message"].toString();
    auto target = obj["target"].toString();
    return Log(timestamp, level_enum, message, target);
}

QString Log::timestamp() const { return m_timestamp; }

void Log::setTimestamp(const QString &timestamp) {
    if (m_timestamp == timestamp) return;
    m_timestamp = timestamp;
}

EventLevel Log::level() const { return m_level; }

QString Log::levelString() const { 
    switch (m_level) {
        case EventLevel::INFO:
            return "INFO";
        case EventLevel::WARNING:
            return "WARN";
        case EventLevel::ERROR:
            return "ERROR";
        case EventLevel::SUCCESS:
            return "SUCCESS";
    }
    return "INFO";
 }

void Log::setLevel(EventLevel level) {
    if (m_level == level) return;
    m_level = level;
}

QString Log::message() const { return m_message; }

void Log::setMessage(const QString &message) {
    if (m_message == message) return;
    m_message = message;
}

QString Log::target() const {
    return m_target;
}

void Log::setTarget(const QString &target) {
    if (m_target == target) return;
    m_target = target;
}

LogList::LogList(QObject *parent) : QAbstractListModel(parent) {}

LogList::~LogList() = default;

int LogList::rowCount(const QModelIndex &parent) const { return m_list.size(); }

QVariant LogList::data(const QModelIndex &index, int role) const {
    if (!index.isValid()) {
        return {};
    }

    if (index.row() >= m_list.size()) {
        return {};
    }

    switch (role) {
        case LevelRole:
            return m_list.at(index.row()).level();
        case TimestampRole:
            return m_list.at(index.row()).timestamp();
        case MessageRole:
            return m_list.at(index.row()).message();
        case TargetRole:
            return m_list.at(index.row()).target();
        default:
            return {};
    }
}

QHash<int, QByteArray> LogList::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[LevelRole] = "level";
    roles[TimestampRole] = "timestamp";
    roles[MessageRole] = "message";
    roles[TargetRole] = "target";
    return roles;
}

void LogList::appendLogs(const QString &json) {
    auto logs_json = json.split("\n");
    for (const auto &log_json : logs_json) {
        if (log_json.isEmpty()) continue;
        beginInsertRows(QModelIndex(), m_list.size(), m_list.size());
        m_list.append(Log::fromJson(log_json));
        endInsertRows();
    }
}

void LogList::appendLog(const Log &log) {
    beginInsertRows(QModelIndex(), m_list.size(), m_list.size());
    m_list.append(log);
    endInsertRows();
}

QVector<Log> *LogList::logs() const {
    return const_cast<QVector<Log> *>(&m_list);
}
