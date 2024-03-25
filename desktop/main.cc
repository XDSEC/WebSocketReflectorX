#include <QApplication>
#include <QFont>
#include <QFontDatabase>

#include "ui.h"
#include "variables.h"

#ifdef Q_OS_UNIX
#include <QSocketNotifier>
#include <sys/socket.h>
#include <sys/types.h>
#include <signal.h>

static int setup_unix_signal_handlers() {
    struct sigaction hup, term, segv, intr;

    hup.sa_handler = Ui::sighupSigHandler;
    sigemptyset(&hup.sa_mask);
    hup.sa_flags = 0;
    hup.sa_flags |= SA_RESTART;

    if (sigaction(SIGHUP, &hup, 0)) return 1;

    term.sa_handler = Ui::sigtermSigHandler;
    sigemptyset(&term.sa_mask);
    term.sa_flags = 0;
    term.sa_flags |= SA_RESTART;

    if (sigaction(SIGTERM, &term, 0)) return 2;

    segv.sa_handler = Ui::sigsegvSigHandler;
    sigemptyset(&segv.sa_mask);
    segv.sa_flags = 0;
    segv.sa_flags |= SA_RESTART;

    if (sigaction(SIGSEGV, &segv, 0)) return 3;

    intr.sa_handler = Ui::sigintSigHandler;

    sigemptyset(&intr.sa_mask);
    intr.sa_flags = 0;
    intr.sa_flags |= SA_RESTART;

    if (sigaction(SIGINT, &intr, 0)) return 4;

    return 0;
}

#endif

int main(int argc, char* argv[]) {
    QApplication app(argc, argv);

    QApplication::setApplicationName("wsrx");
    QApplication::setApplicationDisplayName("WebSocket Reflector X");
    QApplication::setOrganizationDomain("tech.woooo.wsrx");
    QApplication::setOrganizationName("Ret2Shell");
    QApplication::setApplicationVersion(FULL_VERSION);

    QFontDatabase::addApplicationFont(":/resources/fonts/sarasa-mono-sc-regular.ttf");
    auto defaultFont = QFont("Sarasa Mono SC");
    QApplication::setFont(defaultFont);

    setup_unix_signal_handlers();

    Ui::instance()->show();

    return QApplication::exec();
}
