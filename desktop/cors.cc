#include "cors.h"

#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QNetworkAccessManager>
#include <QNetworkReply>
#include <QTimer>

Website::Website(const QString& domain, bool passed) {
    m_domain = domain;
    m_passed = passed;
}

Website::Website(const Website& other) {
    m_domain = other.m_domain;
    m_passed = other.m_passed;
}

Website::Website() {}

Website::~Website() = default;

Website& Website::operator=(const Website& other) {
    m_domain = other.m_domain;
    m_passed = other.m_passed;
    return *this;
}

bool Website::operator<(const Website& other) const {
    if (m_passed == other.m_passed) {
        return m_domain < other.m_domain;
    }
    return m_passed < other.m_passed;
}

bool Website::operator==(const Website& other) const {
    return m_domain == other.m_domain && m_passed == other.m_passed;
}

QString Website::domain() const { return m_domain; }

void Website::setDomain(const QString& domain) { m_domain = domain; }

bool Website::passed() const { return m_passed; }

void Website::setPassed(bool passed) { m_passed = passed; }

QString WebsiteList::apiRoot() { return QString::asprintf("http://127.0.0.1:%d/", m_apiPort); }

QUrl WebsiteList::service(const QString& name) { return QUrl(apiRoot() + name); }

WebsiteList::WebsiteList(QObject* parent, quint16 apiPort) : QAbstractListModel(parent) {
    m_apiPort = apiPort;
    m_network = new QNetworkAccessManager(this);
    m_refreshTimer = new QTimer(this);
    m_refreshTimer->setInterval(1000);
    m_refreshTimer->setSingleShot(false);
    m_refreshTimer->start();
    connect(m_refreshTimer, &QTimer::timeout, this, &WebsiteList::syncSites);
}

WebsiteList::~WebsiteList() {
    if (m_network) {
        m_network->deleteLater();
    }
}

int WebsiteList::rowCount(const QModelIndex& parent) const {
    if (parent.isValid()) {
        return 0;
    }
    return m_list.size();
}

QVariant WebsiteList::data(const QModelIndex& index, int role) const {
    if (!index.isValid()) {
        return QVariant();
    }
    if (index.row() >= m_list.size()) {
        return QVariant();
    }
    if (role == DomainRole) {
        return m_list.at(index.row()).domain();
    } else if (role == PassedRole) {
        return m_list.at(index.row()).passed();
    }
    return QVariant();
}

QHash<int, QByteArray> WebsiteList::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[DomainRole] = "domain";
    roles[PassedRole] = "passed";
    return roles;
}

qsizetype WebsiteList::size() const { return m_list.size(); }

void WebsiteList::pass(const QString& domain) {
    auto request = QNetworkRequest(service("access"));
    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");
    auto reply = m_network->post(request, QString("\"%1\"").arg(domain).toUtf8());
    connect(reply, &QNetworkReply::finished, this, [=]() {
        if (reply->error() != QNetworkReply::NoError) {
            qWarning() << reply->errorString();
            qWarning() << reply->readAll();
        } else {
            syncSites();
        }
        reply->deleteLater();
    });
}

void WebsiteList::deny(const QString& domain) {
    auto request = QNetworkRequest(service("access"));
    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");
    auto reply = m_network->sendCustomRequest(request, "DELETE", QString("\"%1\"").arg(domain).toUtf8());
    connect(reply, &QNetworkReply::finished, this, [=]() {
        if (reply->error() != QNetworkReply::NoError) {
            qWarning() << reply->errorString();
            qWarning() << reply->readAll();
        } else {
            syncSites();
        }
        reply->deleteLater();
    });
}

void WebsiteList::syncSites() {
    auto request = QNetworkRequest(service("access"));
    auto reply = m_network->get(request);
    connect(reply, &QNetworkReply::finished, this, [=]() {
        if (reply->error() != QNetworkReply::NoError) {
            qWarning() << reply->errorString();
            qWarning() << reply->readAll();
        } else {
            auto resp = reply->readAll();
            // qDebug() << "Syncing sites" << resp;
            auto json = QJsonDocument::fromJson(resp).object();
            auto allowed = json["allowed"].toArray();
            auto waiting = json["pending"].toArray();
            auto fetchedList = QList<Website>();
            for (const auto& site : allowed) {
                // qDebug() << site.toString();
                fetchedList.append(Website(site.toString(), true));
            }
            for (const auto& site : waiting) {
                // qDebug() << site.toString();
                fetchedList.append(Website(site.toString(), false));
            }
            auto oldList = m_list;
            // remove old one
            for (const auto& site : oldList) {
                if (!fetchedList.contains(site)) {
                    auto index = oldList.indexOf(site);
                    beginRemoveRows(QModelIndex(), index, index);
                    m_list.removeOne(site);
                    endRemoveRows();
                }
            }
            // add new one
            for (const auto& site : fetchedList) {
                if (!oldList.contains(site)) {
                    beginInsertRows(QModelIndex(), m_list.size(), m_list.size());
                    m_list.append(site);
                    endInsertRows();
                }
            }
            emit sizeChanged(size());
        }
        reply->deleteLater();
    });
}
