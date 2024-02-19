#include "ui.h"
#include "variables.h"
#include <QApplication>
#include <QFont>
#include <QFontDatabase>

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    QApplication::setApplicationName("wsrx");
    QApplication::setApplicationDisplayName("WebSocket Reflector X");
    QApplication::setOrganizationDomain("tech.woooo.wsrx");
    QApplication::setOrganizationName("Ret2Shell");

    QFontDatabase::addApplicationFont(
        ":/resources/fonts/JetBrainsMono-Regular.ttf");
    auto defaultFont = QFont("JetBrains Mono");
    QApplication::setFont(defaultFont);

    auto ui = Ui();
    ui.show();

    return QApplication::exec();
}
