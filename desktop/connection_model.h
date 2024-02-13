#pragma once

#include <QObject>

class ConnectionModel : public QObject {
    Q_OBJECT
    public:
    explicit ConnectionModel(QObject *parent = nullptr, const QString &remoteAddr = "", const QString &wsAddr = "", const QString &tcpAddr = "");
    ~ConnectionModel();
    QString websocketAddress() const;
    QString tcpAddress() const;
    QString remoteAddress() const;
    uint lagency() const;
    private:
    
    QString m_wsAddr;
    QString m_tcpAddr;
    QString m_remoteAddr;
    uint m_lagency;
};