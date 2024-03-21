#include "daemon.h"

#include <QAbstractSocket>
#include <QNetworkInterface>
#include <QSysInfo>

#include "variables.h"

Daemon::Daemon(QObject *parent) : QObject(parent) {
    refreshAvailableAddresses();
}

Daemon::~Daemon() = default;

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