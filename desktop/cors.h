#pragma once

#include <QAbstractListModel>
#include <QObject>

class QNetworkAccessManager;
class QTimer;

class Website : public QObject {
    Q_GADGET
    Q_PROPERTY(QString domain READ domain WRITE setDomain)
    Q_PROPERTY(bool passed READ passed WRITE setPassed)

  private:
    QString m_domain;
    bool m_passed;

  public:
    Website(const QString& domain, bool passed);

    Website(const Website& other);

    Website();

    ~Website() override;

    Website& operator=(const Website& other);

    bool operator<(const Website& other) const;

    bool operator==(const Website& other) const;

    [[nodiscard]] QString domain() const;

    void setDomain(const QString& domain);

    [[nodiscard]] bool passed() const;

    void setPassed(bool passed);
};

Q_DECLARE_METATYPE(Website)

class WebsiteList : public QAbstractListModel {
    Q_OBJECT
    Q_PROPERTY(qsizetype size READ size NOTIFY sizeChanged)
  private:
    QVector<Website> m_list{};
    QNetworkAccessManager* m_network;
    quint16 m_apiPort;
    QTimer* m_refreshTimer;

    QString apiRoot();

    QUrl service(const QString& name);

  public:
    enum WebsiteRoles { DomainRole = Qt::UserRole + 1, PassedRole };

    explicit WebsiteList(QObject* parent = nullptr, quint16 apiPort = 3307);

    ~WebsiteList() override;

    [[nodiscard]] int rowCount(const QModelIndex& parent = QModelIndex()) const override;

    [[nodiscard]] QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;

    [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

    [[nodiscard]] qsizetype size() const;

  public slots:
    Q_INVOKABLE void syncSites();

    void pass(const QString& domain);

    void deny(const QString& domain);

  signals:
    void sizeChanged(qsizetype n);
};
