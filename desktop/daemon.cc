#include "daemon.h"

#include <QAbstractSocket>
#include <QCoreApplication>
#include <QDateTime>
#include <QJsonDocument>
#include <QJsonObject>
#include <QNetworkInterface>
#include <QProcess>
#include <QSysInfo>

#include "variables.h"

Log::Log(const QString &timestamp, LogLevel level, const QString &message, const QString &target)
    : m_timestamp(timestamp), m_level(level), m_message(message), m_target(target) {}

Log::Log(const Log &other) {
    m_timestamp = other.m_timestamp;
    m_level = other.m_level;
    m_message = other.m_message;
    m_target = other.m_target;
}

Log::Log() {
    m_timestamp = QDateTime::currentDateTime().toString(Qt::ISODate);
    m_level = LogLevel::INFO;
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
    LogLevel level_enum;
    if (level == "INFO")
        level_enum = LogLevel::INFO;
    else if (level == "WARN")
        level_enum = LogLevel::WARNING;
    else if (level == "ERROR")
        level_enum = LogLevel::ERROR;
    else
        level_enum = LogLevel::INFO;
    auto message = obj["fields"].toObject()["message"].toString();
    auto target = obj["target"].toString();
    return Log(timestamp, level_enum, message, target);
}

QString Log::timestamp() const { return m_timestamp; }

void Log::setTimestamp(const QString &timestamp) {
    if (m_timestamp == timestamp) return;
    m_timestamp = timestamp;
}

LogLevel Log::level() const { return m_level; }

void Log::setLevel(LogLevel level) {
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

Daemon::Daemon(QObject *parent) : QObject(parent) {
    refreshAvailableAddresses();
    auto daemon_path = QCoreApplication::applicationDirPath() + "/wsrx";
#ifdef Q_OS_WIN
    daemon_path += ".exe";
#endif
    auto args = QStringList{"daemon", "-l", "true", "-p", "3307"};
    m_daemon = new QProcess(this);
    m_daemon->start(daemon_path, args);
    if (!m_daemon->waitForStarted()) {
        qWarning() << "Daemon is not started correctly.";
    }
    m_logs = new LogList(this);

    connect(m_daemon, &QProcess::readyReadStandardOutput, this, [this]() {
        m_logs->appendLogs(m_daemon->readAllStandardOutput());
    });
    connect(m_daemon, &QProcess::readyReadStandardError, this, [this]() {
        qWarning() << m_daemon->readAllStandardError();
    });
}

Daemon::~Daemon() {
    m_daemon->terminate();
    if (!m_daemon->waitForFinished())
        qWarning() << "Daemon is not terminated correctly.";
    m_daemon->deleteLater();
}

QStringList Daemon::availableAddresses() const { return m_availableAddresses; }

void Daemon::setAvailableAddresses(const QStringList &availableAddresses) {
    if (m_availableAddresses == availableAddresses) return;
    m_availableAddresses = availableAddresses;
    emit availableAddressesChanged(availableAddresses);
}

Q_INVOKABLE void Daemon::refreshAvailableAddresses() {
    auto addresses = QNetworkInterface::allAddresses();
    QStringList availableAddresses;
    availableAddresses.append("127.0.0.1");
    for (const auto &address : addresses) {
        if (address.isLoopback()) continue;
        if (address.protocol() != QAbstractSocket::IPv4Protocol) continue;
        availableAddresses.append(address.toString());
    }
    availableAddresses.append("0.0.0.0");
    setAvailableAddresses(availableAddresses);
}

QString Daemon::systemInfo() const {
    auto info =
        QString(
            "System\t: %1\nCPU\t: %2\nKernel\t: %3-%4\nABI\t: %5\nWSRX\t: "
            "%6\nMachine\t: %7-%8")
            .arg(QSysInfo::prettyProductName(),
                 QSysInfo::currentCpuArchitecture(), QSysInfo::kernelType(),
                 QSysInfo::kernelVersion(), QSysInfo::buildAbi(), FULL_VERSION,
                 QSysInfo::machineHostName(), QSysInfo::machineUniqueId());
#ifdef Q_OS_LINUX
    info.append(
        QString("\nDesktop\t: %1-%2")
            .arg(qgetenv("XDG_CURRENT_DESKTOP"), qgetenv("XDG_SESSION_TYPE")));
#endif
    return info;
}

LogList *Daemon::logs() const {
    return m_logs;
}
