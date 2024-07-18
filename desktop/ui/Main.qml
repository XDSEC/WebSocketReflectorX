import QtQuick
import QtQuick.Controls
import QtQuick.Effects
import QtQuick.Layouts
import Rx.Widgets

FramelessWindow {
    id: window

    minimumHeight: 700
    minimumWidth: 1200
    objectName: "mainWindow"
    visible: true
    width: 1200
    height: 700
    Component.onCompleted: {
        setX(Screen.width / 2 - width / 2);
        setY(Screen.height / 2 - height / 2);
        Style.isDark = ui.isDark;
    }

    SystemTray {
    }
    
    KeyTapEvent {
        id: exitAppEvent

        customKey: "Ctrl+Q"
        onClicked: {
            ui.requestToQuit();
        }
    }

    Rectangle {
        id: centralWidget

        anchors.fill: parent
        anchors.margins: ui.isMac ? 0 : (window.visibility === Window.Windowed ? 10 : 0)
        border.color: Style.palette.mid
        border.width: ui.isMac ? 0 : (window.visibility === Window.Windowed ? 1 : 0)
        color: Style.palette.window

        Item {
            anchors.fill: parent
            anchors.margins: ui.isMac ? 0 : (window.visibility === Window.Windowed ? 1 : 0)

            SplitView {
                anchors.fill: parent
                orientation: Qt.Horizontal

                SideBar {
                    id: sideBar

                    SplitView.preferredWidth: 280
                    SplitView.minimumWidth: 200
                    SplitView.maximumWidth: 400

                    Connections {
                        function onConnected(success, message) {
                            if (success)
                                sideBar.page = 1;

                        }

                        target: daemon
                    }

                }

                Item {
                    TitleBar {
                        id: titleBar

                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                    }

                    SwipeView {
                        id: stack

                        clip: true
                        anchors.bottom: parent.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.top: titleBar.bottom
                        currentIndex: sideBar.page
                        orientation: Qt.Vertical
                        interactive: false

                        GetStartedView {
                            id: getStartedView
                        }

                        ConnectionsView {
                            id: connectionsView
                        }

                        LogsView {
                            id: logsView
                        }

                        WebsitesView {
                            id: websitesView
                        }

                        SettingsView {
                            id: settingsView
                        }

                    }

                }

            }

        }

    }

}
