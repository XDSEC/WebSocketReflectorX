#include "pool.h"

#include <QElapsedTimer>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QNetworkAccessManager>
#include <QNetworkReply>

#include "log.h"

Link::Link(const QString& from, const QString& to, LinkStatus status, quint32 latency)
    : m_from(from), m_to(to), m_status(status), m_latency(latency) {}

Link::Link(const Link& other) {
    m_from = other.m_from;
    m_to = other.m_to;
    m_status = other.m_status;
    m_latency = other.m_latency;
}

Link::Link() {
    m_from = "EMPTY";
    m_to = "EMPTY";
    m_status = LinkStatus::POOLING;
    m_latency = 0;
}

Link::~Link() = default;

Link& Link::operator=(const Link& other) {
    if (this == &other) return *this;
    m_from = other.m_from;
    m_to = other.m_to;
    m_status = other.m_status;
    m_latency = other.m_latency;
    return *this;
}

bool Link::operator<(const Link& other) const {
    if (m_status == other.m_status) {
        if (m_latency == other.m_latency) {
            if (m_from == other.m_from) {
                return m_to < other.m_to;
            }
            return m_from < other.m_from;
        }
        return m_latency < other.m_latency;
    }
    return m_status < other.m_status;
}

Link Link::fromJson(const QString& json) {
    auto doc = QJsonDocument::fromJson(json.toUtf8());
    auto obj = doc.object();
    auto from = obj["from"].toString();
    auto to = obj["to"].toString();
    return Link(from, to, POOLING, 0);
}

QString Link::from() const { return m_from; }

void Link::setFrom(const QString& from) {
    if (m_from == from) return;
    m_from = from;
}

QString Link::to() const { return m_to; }

void Link::setTo(const QString& to) {
    if (m_to == to) return;
    m_to = to;
}

LinkStatus Link::status() const { return m_status; }

void Link::setStatus(LinkStatus status) {
    if (m_status == status) return;
    m_status = status;
}

quint32 Link::latency() const { return m_latency; }

void Link::setLatency(quint32 latency) {
    if (m_latency == latency) return;
    m_latency = latency;
}

LinkList::LinkList(QObject* parent) : QAbstractListModel(parent) {
    m_network = new QNetworkAccessManager(this);
    connect(this, &LogList::dataChanged, this, [=]() { emit sizeChanged(rowCount(QModelIndex())); });
}

LinkList::~LinkList() = default;

void LinkList::setLogs(LogList* logs) { m_logs = logs; }

int LinkList::rowCount(const QModelIndex& parent) const {
    if (parent.isValid()) return 0;
    return m_list.size();
}

QVariant LinkList::data(const QModelIndex& index, int role) const {
    if (!index.isValid()) return QVariant();
    if (index.row() >= m_list.size()) return QVariant();
    if (role == FromRole) return m_list.at(index.row()).from();
    if (role == ToRole) return m_list.at(index.row()).to();
    if (role == StatusRole) return m_list.at(index.row()).status();
    if (role == DelayRole) return m_list.at(index.row()).latency();
    return QVariant();
}

bool LinkList::setData(const QModelIndex& index, const QVariant& value, int role) {
    if (!index.isValid()) return false;
    if (index.row() >= m_list.size()) return false;
    if (role == FromRole) {
        m_list[index.row()].setFrom(value.toString());
    } else if (role == ToRole) {
        m_list[index.row()].setTo(value.toString());
    } else if (role == StatusRole) {
        m_list[index.row()].setStatus(static_cast<LinkStatus>(value.toInt()));
    } else if (role == DelayRole) {
        m_list[index.row()].setLatency(value.toUInt());
    } else {
        return false;
    }
    emit dataChanged(index, index, {role});
    return true;
}

QHash<int, QByteArray> LinkList::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[FromRole] = "from";
    roles[ToRole] = "to";
    roles[StatusRole] = "status";
    roles[DelayRole] = "latency";
    return roles;
}

void LinkList::syncLinks(const QString& json) {
    // qDebug() << "syncLinks" << json;
    auto doc = QJsonDocument::fromJson(json.toUtf8());
    auto arr = doc.object();
    auto newLinks = QVector<Link>{};
    for (const auto& val : arr.keys()) {
        auto obj = arr.value(val).toObject();
        // qDebug() << "obj" << obj;
        auto from = obj["from"].toString();
        auto to = obj["to"].toString();
        newLinks.append(Link(from, to, POOLING, 0));
    }
    // remove old links from m_list which are not in newLinks
    for (auto i = 0; i < m_list.size(); ++i) {
        auto found = false;
        for (const auto& link : newLinks) {
            if (m_list.at(i).from() == link.from() && m_list.at(i).to() == link.to()) {
                found = true;
                break;
            }
        }
        if (!found) {
            beginRemoveRows(QModelIndex(), i, i);
            m_list.removeAt(i);
            endRemoveRows();
        }
    }
    // add new links to m_list which are not in m_list
    for (const auto& link : newLinks) {
        auto found = false;
        for (const auto& oldLink : m_list) {
            if (oldLink.from() == link.from() && oldLink.to() == link.to()) {
                found = true;
                break;
            }
        }
        if (!found) {
            beginInsertRows(QModelIndex(), m_list.size(), m_list.size());
            m_list.append(link);
            endInsertRows();
        }
    }
    emit sizeChanged(rowCount(QModelIndex()));
    refreshStatus();
}

void LinkList::refreshStatus() {
    for (auto i = 0; i < m_list.size(); ++i) {
        auto link = m_list.at(i);
        // qDebug() << "refreshStatus" << link.from() << link.to();
        if (link.status() == LinkStatus::DEAD) continue;
        auto url = QUrl(link.to());
        if (url.scheme() == "ws")
            url.setScheme("http");
        else if (url.scheme() == "wss")
            url.setScheme("https");
        auto request = QNetworkRequest(url);
        QElapsedTimer timer;
        timer.start();
        auto reply = m_network->sendCustomRequest(request, "OPTIONS");
        connect(reply, &QNetworkReply::finished, this, [=]() {
            auto latency = timer.elapsed();
            if (reply->error() != QNetworkReply::NoError) {
                if (m_logs) {
                    m_logs->appendLog(Log(QDateTime::currentDateTimeUtc().toString(Qt::ISODate), EventLevel::ERROR,
                                          reply->errorString(), u"wsrx::desktop::pool"_qs));
                }
                setData(index(i), LinkStatus::DEAD, StatusRole);
            } else {
                // m_list[i].setLatency(latency);
                // m_list[i].setStatus(LinkStatus::ALIVE);
                setData(index(i), latency, DelayRole);
                setData(index(i), LinkStatus::ALIVE, StatusRole);
            }
            reply->deleteLater();
        });
    }
}

void LinkList::clear() {
    beginRemoveRows(QModelIndex(), 0, m_list.size());
    m_list.clear();
    endRemoveRows();
    emit sizeChanged(rowCount(QModelIndex()));
}

int LinkList::size() const { return rowCount(QModelIndex()); }
