#pragma once

#include <QCloseEvent>
#include <QObject>

class QQmlEngine;
class QQmlComponent;
class QQuickWindow;
class QTranslator;
class QNetworkAccessManager;
#ifdef Q_OS_UNIX
class QSocketNotifier;
#endif
class Daemon;
class ToastList;
class WebsiteList;

class Ui : public QObject {
    Q_OBJECT
    Q_PROPERTY(bool runningInTray READ runningInTray WRITE setRunningInTray NOTIFY runningInTrayChanged)
    Q_PROPERTY(bool isDark READ isDark WRITE setIsDark NOTIFY isDarkChanged)
    Q_PROPERTY(QString language READ language WRITE setLanguage NOTIFY languageChanged)
    Q_PROPERTY(bool isMac READ isMac WRITE setIsMac NOTIFY isMacChanged)
    Q_PROPERTY(QString version READ version NOTIFY versionChanged)
    Q_PROPERTY(bool hasNewVersion READ hasNewVersion NOTIFY hasNewVersionChanged)
    Q_PROPERTY(QString newVersion READ newVersion NOTIFY newVersionChanged)
    Q_PROPERTY(QString updateUrl READ updateUrl NOTIFY updateUrlChanged)
  private:
    static Ui* m_instance;
    QQmlEngine* m_uiEngine;
    QQmlComponent* m_uiComponent;
    QQuickWindow* m_window{};
    QTranslator* m_translator{};
    Daemon* m_daemon;
    ToastList* m_toasts;
    WebsiteList* m_websites;
    bool m_runningInTray = false;
    bool m_isDark = false;
    QString m_language = "en_US";
    bool m_isMac = false;
    QNetworkAccessManager* m_networkManager;
    QString m_version;
    QString m_newVersion;
    QString m_updateUrl;
    bool m_hasNewVersion = false;

#ifdef Q_OS_UNIX
    static int sighupFd[2];
    static int sigtermFd[2];
    static int sigsegvFd[2];
    static int sigintFd[2];
    QSocketNotifier* snHup;
    QSocketNotifier* snTerm;
    QSocketNotifier* snSegv;
    QSocketNotifier* snInt;
#endif
  protected:
    void loadSettings();

    void saveSettings();

    explicit Ui(QObject* parent = nullptr);

    ~Ui() override;

  public:
    static Ui* instance(QObject* parent = nullptr);

    [[nodiscard]] bool runningInTray() const;

    void setRunningInTray(bool runningInTray);

    [[nodiscard]] bool isDark() const;

    void setIsDark(bool isDark);

    [[nodiscard]] QString language() const;

    void setLanguage(const QString& language);

    [[nodiscard]] bool isMac() const;

    void setIsMac(bool isMac);

    [[nodiscard]] QString version() const;

    void setVersion(const QString& version);

    [[nodiscard]] bool hasNewVersion() const;

    void setHasNewVersion(bool hasNewVersion);

    [[nodiscard]] QString newVersion() const;

    void setNewVersion(const QString& newVersion);

    [[nodiscard]] QString updateUrl() const;

    void setUpdateUrl(const QString& updateUrl);

#ifdef Q_OS_UNIX

    static void sigtermSigHandler(int);

    static void sighupSigHandler(int);

    static void sigsegvSigHandler(int);

    static void sigintSigHandler(int);

  public slots:
    void sigtermHandler();

    void sighupHandler();

    void sigsegvHandler();

    void sigintHandler();
#endif

  public slots:
    Q_INVOKABLE void show();

    Q_INVOKABLE void requestToQuit();

    Q_INVOKABLE void onSecondaryInstanceMessageReceived(
        quint32 instanceId, const QByteArray& message);

    Q_INVOKABLE void onSecondaryInstanceStarted();

    Q_INVOKABLE void checkUpdates();

  signals:
    void pageChanged(quint8 page);

    void runningInTrayChanged(bool runningInTray);

    void isDarkChanged(bool isDark);

    void languageChanged(const QString& language);

    void isMacChanged(bool isMac);

    void versionChanged(const QString& version);

    void hasNewVersionChanged(bool hasNewVersion);

    void newVersionChanged(const QString& newVersion);

    void updateUrlChanged(const QString& updateUrl);
};
