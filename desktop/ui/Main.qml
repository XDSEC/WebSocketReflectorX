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
    }

    Rectangle {
        id: centralWidget

        anchors.fill: parent
        anchors.margins: window.visibility === Window.Windowed ? 10 : 0
        border.color: Style.palette.mid
        border.width: window.visibility === Window.Windowed ? 1 : 0
        color: Style.palette.window

        Item {
            anchors.fill: parent
            anchors.margins: window.visibility === Window.Windowed ? 1 : 0

            SplitView {
                anchors.fill: parent
                orientation: Qt.Horizontal

                SideBar {
                    id: sideBar

                    SplitView.preferredWidth: 280
                    SplitView.minimumWidth: 200
                    SplitView.maximumWidth: 400
                }

                Item {
                    TitleBar {
                        id: titleBar

                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                    }

                    StackLayout {
                        id: stack

                        anchors.bottom: parent.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.top: titleBar.bottom
                        currentIndex: ui.page

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
