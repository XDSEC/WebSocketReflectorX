#pragma once

#include "api.h"
#include <QCloseEvent>
#include <QObject>

class QQmlEngine;
class QQmlComponent;
class QQuickWindow;
class QTranslator;

class Ui : public QObject {
    Q_OBJECT
    Q_PROPERTY(quint8 page READ page WRITE setPage NOTIFY pageChanged)
    Q_PROPERTY(QStringList availableAddresses READ availableAddresses WRITE
                   setAvailableAddresses NOTIFY availableAddressesChanged)
  private:
    QQmlEngine *m_uiEngine;
    QQmlComponent *m_uiComponent;
    QQuickWindow *m_window{};
    QTranslator *m_translator{};
    quint8 m_page = 0;
    QStringList m_availableAddresses{"127.0.0.1", "0.0.0.0"};
    Api *m_api;

  public:
    explicit Ui(QObject *parent = nullptr);

    ~Ui() override;

    [[nodiscard]] quint8 page() const;
    void setPage(quint8 page);

    [[nodiscard]] QStringList availableAddresses() const;
    void setAvailableAddresses(const QStringList &availableAddresses);

    [[nodiscard]] QString address() const;
    void setAddress(const QString &address);

    [[nodiscard]] quint16 port() const;
    void setPort(quint16 port);

  public slots:
    Q_INVOKABLE void show();
    Q_INVOKABLE void refreshAvailableAddresses();
    Q_INVOKABLE void requestToQuit();

  signals:
    void pageChanged(quint8 page);
    void availableAddressesChanged(const QStringList &availableAddresses);
};
