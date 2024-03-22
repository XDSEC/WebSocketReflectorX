#include "daemon.h"

#include <QAbstractSocket>
#include <QCoreApplication>
#include <QNetworkInterface>
#include <QProcess>
#include <QSysInfo>

#include "variables.h"

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