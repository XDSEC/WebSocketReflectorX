#pragma once

#include <QAbstractListModel>
#include <QObject>

class QNetworkAccessManager;
class LogList;

enum LinkStatus { POOLING, ALIVE, DEAD };

class Link : public QObject {
    Q_GADGET
    Q_PROPERTY(QString from READ from WRITE setFrom)
    Q_PROPERTY(QString to READ to WRITE setTo)
    Q_PROPERTY(LinkStatus status READ status WRITE setStatus)
    Q_PROPERTY(quint32 latency READ latency WRITE setLatency)

   private:
    QString m_from;
    QString m_to;
    LinkStatus m_status;
    quint32 m_latency;

   public:
    Link(const QString &from, const QString &to, LinkStatus status,
         quint32 latency);

    Link(const Link &other);

    Link();

    ~Link() override;

    Link &operator=(const Link &other);

    static Link fromJson(const QString &json);

    [[nodiscard]] QString from() const;

    void setFrom(const QString &from);

    [[nodiscard]] QString to() const;

    void setTo(const QString &to);

    [[nodiscard]] LinkStatus status() const;

    void setStatus(LinkStatus status);

    [[nodiscard]] quint32 latency() const;

    void setLatency(quint32 latency);
};

Q_DECLARE_METATYPE(Link)

class LinkList : public QAbstractListModel {
    Q_OBJECT

   private:
    QVector<Link> m_list{};
    QNetworkAccessManager *m_network;
    LogList* m_logs;

   public:
    enum LinkRoles {
        FromRole = Qt::UserRole + 1,
        ToRole,
        StatusRole,
        DelayRole
    };
    explicit LinkList(QObject *parent = nullptr);

    ~LinkList() override;

    void setLogs(LogList* logs);

    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role) const override;

    QHash<int, QByteArray> roleNames() const override;

    void syncLinks(const QString &json);

    void refreshStatus();

    void clear();
};
