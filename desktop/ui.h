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
    Q_PROPERTY(bool runningInTray READ runningInTray WRITE setRunningInTray NOTIFY runningInTrayChanged)
    Q_PROPERTY(bool isDark READ isDark WRITE setIsDark NOTIFY isDarkChanged)
    Q_PROPERTY(QString language READ language WRITE setLanguage NOTIFY languageChanged)
  private:
    QQmlEngine *m_uiEngine;
    QQmlComponent *m_uiComponent;
    QQuickWindow *m_window{};
    QTranslator *m_translator{};
    Daemon *m_daemon;
    ToastList *m_toasts;
    bool m_runningInTray = false;
    bool m_isDark = false;
    QString m_language = "zh_CN";

  protected:
    void loadSettings();

    void saveSettings();

  public:
    explicit Ui(QObject *parent = nullptr);

    ~Ui() override;

    [[nodiscard]] bool runningInTray() const;

    void setRunningInTray(bool runningInTray);

    [[nodiscard]] bool isDark() const;

    void setIsDark(bool isDark);

    [[nodiscard]] QString language() const;

    void setLanguage(const QString &language);

  public slots:
    Q_INVOKABLE void show();

    Q_INVOKABLE void requestToQuit();

  signals:
    void pageChanged(quint8 page);

    void runningInTrayChanged(bool runningInTray);

    void isDarkChanged(bool isDark);

    void languageChanged(const QString &language);
};
