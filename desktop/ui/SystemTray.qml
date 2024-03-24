import Qt.labs.platform
import QtQuick

SystemTrayIcon {
    id: systray

    icon.source: "qrc:/resources/assets/logo.svg"
    visible: true
    onActivated: function (reason) {
        if (reason === SystemTrayIcon.Context)
            menu.open();
        else {
            window.show();
            window.raise();
            window.requestActivate();
        }
    }

    menu: Menu {
        id: menu

        visible: false

        MenuItem {
            text: qsTr("Tunnels")
            onTriggered: {
                sideBar.page = 1;
                window.show();
                window.raise();
                window.requestActivate();
            }
        }

        MenuItem {
            text: qsTr("Network Logs")
            onTriggered: {
                sideBar.page = 2;
                window.show();
                window.raise();
                window.requestActivate();
            }
        }

        MenuItem {
            text: qsTr("Permitted Websites")
            onTriggered: {
                sideBar.page = 3;
                window.show();
                window.raise();
                window.requestActivate();
            }
        }

        MenuItem {
            text: qsTr("Quit")
            onTriggered: ui.requestToQuit()
        }
    }
}
