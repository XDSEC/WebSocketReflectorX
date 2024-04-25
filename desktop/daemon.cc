#include "daemon.h"

#include <QAbstractSocket>
#include <QCoreApplication>
#include <QDateTime>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>
#include <QNetworkAccessManager>
#include <QNetworkInterface>
#include <QNetworkReply>
#include <QProcess>
#include <QSysInfo>
#include <QTimer>

#include "log.h"
#include "pool.h"
#include "variables.h"

Daemon::Daemon(QObject* parent) : QObject(parent) {
    refreshAvailableAddresses();
    auto daemon_path = QCoreApplication::applicationDirPath() + "/wsrx";
#ifdef Q_OS_WIN
    daemon_path += ".exe";
#endif
    m_logs = new LogList(this);
    m_links = new LinkList(this);

    auto args = QStringList{"daemon", "-l", "true", "-p", "3307", "--heartbeat", "3"};
    m_daemon = new QProcess(this);
    m_daemon->start(daemon_path, args);
    if (!m_daemon->waitForStarted()) {
        qWarning() << "Daemon is not started correctly.";
        m_logs->appendLog(
            Log(QDateTime::currentDateTime().toString(Qt::ISODate), EventLevel::ERROR,
                tr("Failed to start daemon: ") + m_daemon->errorString() + " " + m_daemon->readAllStandardError(),
                "wsrx::desktop::connector"));
    }
    m_links->setLogs(m_logs);
    m_refreshTimer = new QTimer(this);
    m_refreshTimer->setInterval(30 * 1000);
    m_refreshTimer->start();
    m_heartbeatTimer = new QTimer(this);
    m_heartbeatTimer->setInterval(1 * 1000);
    m_heartbeatTimer->start();

    connect(m_refreshTimer, &QTimer::timeout, this, [this]() { syncPool(); });
    connect(m_heartbeatTimer, &QTimer::timeout, this, [this]() { heartbeat(); });

    connect(m_daemon, &QProcess::readyReadStandardOutput, this,
            [this]() { m_logs->appendLogs(m_daemon->readAllStandardOutput()); });
    connect(m_daemon, &QProcess::readyReadStandardError, this, [this]() {
        auto errorString = m_daemon->readAllStandardError();
        qWarning() << errorString;
        m_logs->appendLog(Log(QDateTime::currentDateTime().toString(Qt::ISODate), EventLevel::ERROR, errorString,
                              "wsrx::desktop::connector"));
    });
    connect(this, &Daemon::connected, this, [this](bool success, const QString& _message) {
        if (success) syncPool();
    });
    m_network = new QNetworkAccessManager(this);
}

Daemon::~Daemon() {
#ifdef Q_OS_WIN
    m_daemon->kill();
#else
    m_daemon->terminate();
#endif
    if (!m_daemon->waitForFinished()) {
        qWarning() << "Daemon is not terminated correctly by SIGTERM, try SIGKILL.";
        m_daemon->kill();
    }
    m_daemon->deleteLater();
}

QStringList Daemon::availableAddresses() const { return m_availableAddresses; }

void Daemon::setAvailableAddresses(const QStringList& availableAddresses) {
    if (m_availableAddresses == availableAddresses) return;
    m_availableAddresses = availableAddresses;
    emit availableAddressesChanged(availableAddresses);
}

Q_INVOKABLE void Daemon::refreshAvailableAddresses() {
    auto addresses = QNetworkInterface::allAddresses();
    QStringList availableAddresses;
    availableAddresses.append("127.0.0.1");
    for (const auto& address : addresses) {
        if (address.isLoopback()) continue;
        if (address.protocol() != QAbstractSocket::IPv4Protocol) continue;
        availableAddresses.append(address.toString());
    }
    availableAddresses.append("0.0.0.0");
    setAvailableAddresses(availableAddresses);
}

Q_INVOKABLE void Daemon::requestConnect(const QString& address, const QString& host, const quint16 port) {
    auto parsed = QUrl(address);
    if (!parsed.isValid()) {
        emit connected(false, tr("Invalid URL format."));
        return;
    } else if (parsed.scheme() != "ws" && parsed.scheme() != "wss") {
        emit connected(false, tr("Invalid scheme, only `ws` and `wss` are supported."));
        return;
    }
    auto url = QUrl(m_apiRoot + "pool");
    auto request = QNetworkRequest(url);
    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");
    auto json = QJsonObject();
    json["to"] = address;
    json["from"] = QString("%1:%2").arg(host).arg(port);
    // qDebug() << QJsonDocument(json).toJson();
    auto reply = m_network->post(request, QJsonDocument(json).toJson());
    connect(reply, &QNetworkReply::finished, this, [=]() {
        if (reply->error() != QNetworkReply::NoError) {
            // qDebug() << reply->errorString();
            emit connected(false, reply->readAll());
        } else {
            // qDebug() << reply->readAll();
            emit connected(true, reply->readAll());
        }
        reply->deleteLater();
    });
}

Q_INVOKABLE void Daemon::requestDisconnect(const QString& local_address) {
    auto url = QUrl(m_apiRoot + "pool");
    auto request = QNetworkRequest(url);
    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");
    auto json = QJsonObject();
    json["key"] = local_address;
    auto reply = m_network->sendCustomRequest(request, "DELETE", QJsonDocument(json).toJson());
    connect(reply, &QNetworkReply::finished, this, [=]() {
        if (reply->error() != QNetworkReply::NoError) {
            // qDebug() << reply->errorString();
            emit disconnected(false, reply->readAll());
        } else {
            // qDebug() << reply->readAll();
            emit disconnected(true, reply->readAll());
        }
        syncPool();
        reply->deleteLater();
    });
}

QString Daemon::systemInfo() const {
    auto info = QString("System\t: %1\nCPU\t: %2\nKernel\t: %3-%4\nABI\t: %5\nWSRX\t: "
                        "%6\nMachine\t: %7-%8")
                    .arg(QSysInfo::prettyProductName(), QSysInfo::currentCpuArchitecture(), QSysInfo::kernelType(),
                         QSysInfo::kernelVersion(), QSysInfo::buildAbi(), FULL_VERSION, QSysInfo::machineHostName(),
                         QSysInfo::machineUniqueId());
#ifdef Q_OS_LINUX
    info.append(QString("\nDesktop\t: %1-%2").arg(qgetenv("XDG_CURRENT_DESKTOP"), qgetenv("XDG_SESSION_TYPE")));
#endif
    return info;
}

LogList* Daemon::logs() const { return m_logs; }

LinkList* Daemon::links() const { return m_links; }

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
    file->close();
}

void Daemon::syncPool() {
    auto url = QUrl(m_apiRoot + "pool");
    auto request = QNetworkRequest(url);
    auto reply = m_network->get(request);
    connect(reply, &QNetworkReply::finished, this, [=]() {
        if (reply->error() != QNetworkReply::NoError) {
            qWarning() << reply->errorString();
            qWarning() << reply->readAll();
        } else {
            // qDebug() << reply->readAll();
            m_links->syncLinks(reply->readAll());
        }
        reply->deleteLater();
    });
}

void Daemon::heartbeat() {
    auto url = QUrl(m_apiRoot + "heartbeat");
    auto request = QNetworkRequest(url);
    auto reply = m_network->get(request);
    connect(reply, &QNetworkReply::finished, this, [=]() {
        if (reply->error() != QNetworkReply::NoError) {
            qWarning() << reply->errorString();
            qWarning() << reply->readAll();
        } else {
            // qDebug() << reply->readAll();
        }
        reply->deleteLater();
    });
}

void Daemon::checkOrigins() {}
