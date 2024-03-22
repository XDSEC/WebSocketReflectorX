#include "daemon.h"

#include <QAbstractSocket>
#include <QCoreApplication>
#include <QDateTime>
#include <QJsonDocument>
#include <QJsonObject>
#include <QNetworkInterface>
#include <QProcess>
#include <QSysInfo>
#include <QFile>
#include <QNetworkAccessManager>
#include <QNetworkReply>

#include "variables.h"

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

QVector<Log> *LogList::logs() const {
    return const_cast<QVector<Log> *>(&m_list);
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
    m_network = new QNetworkAccessManager(this);
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

Q_INVOKABLE void Daemon::requestConnect(const QString &address,
                                        const QString &host,
                                        const quint16 port) {
    auto url = QUrl(m_apiRoot + "pool");
    auto request = QNetworkRequest(url);
    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");
    auto json = QJsonObject();
    json["to"] = address;
    json["from"] = QString("%1:%2").arg(host).arg(port);
    // qDebug() << QJsonDocument(json).toJson();
    auto reply = m_network->post(request, QJsonDocument(json).toJson());
    connect(reply, &QNetworkReply::finished, this, [&]() {
        if (reply->error() != QNetworkReply::NoError) {
            emit connected(false, reply->errorString());
        } else {
            emit connected(true, reply->readAll());
        }
        reply->deleteLater();
    });
}

Q_INVOKABLE void Daemon::requestDisconnect(const QString &local_address) {
    return Q_INVOKABLE void();
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

void Daemon::exportLogs(const QUrl& path) const {
    auto file = new QFile(path.toLocalFile());
    if (!file->open(QIODevice::WriteOnly | QIODevice::Text)) {
        qWarning() << "Failed to open file for writing:" << path;
        return;
    }
    QTextStream out(file);
    auto logs = this->m_logs->logs();
    for (const auto& log : *logs) {
        out << log.timestamp() << " [" << log.target() << "] " << log.levelString() << " " << log.message() << "\n";
    }
}
