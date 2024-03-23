import Qt.labs.platform
import QtQuick
import QtQuick.Controls
import Rx.Widgets

Rectangle {
    id: root

    property int page: 0

    color: Style.palette.midlight

    Button {
        id: titleButton

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        height: 48
        alignment: Qt.AlignVCenter | Qt.AlignLeft
        display: AbstractButton.TextBesideIcon
        flat: true
        radius: 0
        icon.source: "qrc:/resources/assets/logo.svg"
        icon.color: "transparent"
        icon.width: 24
        icon.height: 24
        text: "WEBSOCKET REFLECTOR X"
        font.bold: true
        hoverColor: "transparent"
        pressedColor: "transparent"
    }

    FileDialog {
        id: exportLogDialog

        fileMode: FileDialog.SaveFile
        folder: StandardPaths.writableLocation(StandardPaths.DocumentsLocation)
        nameFilters: [qsTr("Text files (*.txt)")]
        onAccepted: {
            daemon.exportLogs(exportLogDialog.file);
        }
    }

    Column {
        spacing: 4
        anchors.top: titleButton.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: settingsTab.top
        anchors.margins: 4

        VerticalTab {
            id: homeTab

            icon.source: "qrc:/resources/assets/home.svg"
            text: qsTr("Get Started")
            width: parent.width
            height: 36
            isCurrent: root.page === 0
            onClicked: {
                root.page = 0;
            }
        }

        VerticalTab {
            id: connectionsTab

            icon.source: "qrc:/resources/assets/link.svg"
            text: qsTr("Connections")
            width: parent.width
            height: 36
            isCurrent: root.page === 1
            onClicked: {
                root.page = 1;
            }
        }

        VerticalTab {
            id: logsTab

            icon.source: "qrc:/resources/assets/code.svg"
            text: qsTr("Network Logs")
            width: parent.width
            height: 36
            isCurrent: root.page === 2
            onClicked: {
                root.page = 2;
            }

            Button {
                id: exportButton

                icon.source: "qrc:/resources/assets/open.svg"
                display: AbstractButton.IconOnly
                anchors.verticalCenter: parent.verticalCenter
                anchors.right: parent.right
                flat: true
                opacity: logsTab.hovered ? 1 : 0
                onClicked: {
                    exportLogDialog.open();
                }

                ToolTip {
                    text: qsTr("Export Logs")
                    visible: exportButton.hovered
                }

                Behavior on opacity {
                    NumberAnimation {
                        duration: Style.midAnimationDuration
                    }

                }

            }

        }

        VerticalTab {
            id: websitesTab

            icon.source: "qrc:/resources/assets/shield.svg"
            text: qsTr("Permitted Websites")
            width: parent.width
            height: 36
            isCurrent: root.page === 3
            onClicked: {
                root.page = 3;
            }
        }

    }

    VerticalTab {
        id: settingsTab

        anchors.bottom: parent.bottom
        anchors.right: parent.right
        anchors.left: parent.left
        anchors.margins: 4
        anchors.bottomMargin: 6
        icon.source: "qrc:/resources/assets/settings.svg"
        text: qsTr("Settings")
        width: parent.width
        height: 36
        isCurrent: root.page === 4
        onClicked: {
            root.page = 4;
        }
    }

}
