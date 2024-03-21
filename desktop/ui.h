#pragma once

#include <QCloseEvent>
#include <QObject>

class QQmlEngine;
class QQmlComponent;
class QQuickWindow;
class QTranslator;
class Daemon;

class Ui : public QObject {
    Q_OBJECT
    Q_PROPERTY(quint8 page READ page WRITE setPage NOTIFY pageChanged)
  private:
    QQmlEngine *m_uiEngine;
    QQmlComponent *m_uiComponent;
    QQuickWindow *m_window{};
    QTranslator *m_translator{};
    Daemon *m_daemon;
    quint8 m_page = 0;

  public:
    explicit Ui(QObject *parent = nullptr);

    ~Ui() override;

    [[nodiscard]] quint8 page() const;
    void setPage(quint8 page);

  public slots:
    Q_INVOKABLE void show();
    Q_INVOKABLE void requestToQuit();

  signals:
    void pageChanged(quint8 page);
};
