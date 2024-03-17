#include <QApplication>
#include <QFont>
#include <QFontDatabase>

#include "ui.h"
#include "variables.h"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    QApplication::setApplicationName("wsrx");
    QApplication::setApplicationDisplayName("WebSocket Reflector X");
    QApplication::setOrganizationDomain("tech.woooo.wsrx");
    QApplication::setOrganizationName("Ret2Shell");

    QFontDatabase::addApplicationFont(
        ":/resources/fonts/sarasa-mono-sc-regular.ttf");
    auto defaultFont = QFont("Sarasa Mono SC");
    QApplication::setFont(defaultFont);

    auto ui = Ui();
    ui.show();

    return QApplication::exec();
}
