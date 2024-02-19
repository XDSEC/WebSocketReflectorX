#pragma once

#include <QObject>

class ConnectionModel : public QObject {
    Q_OBJECT
  public:
    explicit ConnectionModel(QObject *parent = nullptr,
                             const QString &remoteAddr = "",
                             const QString &wsAddr = "",
                             const QString &tcpAddr = "",
                             const qint8 latency = -1);
    ~ConnectionModel();
    QString websocketAddress() const;
    QString tcpAddress() const;
    QString remoteAddress() const;
    qint8 latency() const;

    void setLatency(qint8 latency);

  private:
    QString m_wsAddr;
    QString m_tcpAddr;
    QString m_remoteAddr;
    qint8 m_latency;
};