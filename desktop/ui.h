#pragma once

#include <QCloseEvent>
#include <QObject>

class QQmlEngine;
class QQmlComponent;
class QQuickWindow;
class QTranslator;
class Daemon;
class ToastList;

class Ui : public QObject {
    Q_OBJECT
  private:
    QQmlEngine *m_uiEngine;
    QQmlComponent *m_uiComponent;
    QQuickWindow *m_window{};
    QTranslator *m_translator{};
    Daemon *m_daemon;
    ToastList *m_toasts;

  public:
    explicit Ui(QObject *parent = nullptr);

    ~Ui() override;

  public slots:
    Q_INVOKABLE void show();
    Q_INVOKABLE void requestToQuit();

  signals:
    void pageChanged(quint8 page);
};
