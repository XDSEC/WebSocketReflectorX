import QtQuick
import QtQuick.Controls
import Rx.Widgets

Rectangle {
    id: root

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

    Column {
        spacing: 4
        anchors.top: titleButton.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: aboutTab.top
        anchors.margins: 4

        VerticalTab {
            id: homeTab

            icon.source: "qrc:/resources/assets/home.svg"
            text: qsTr("Get Started")
            width: parent.width
            height: 36
            isCurrent: ui.page === 0
            onClicked: {
                ui.page = 0;
            }
        }

        VerticalTab {
            id: connectionsTab

            icon.source: "qrc:/resources/assets/link.svg"
            text: qsTr("Connections")
            width: parent.width
            height: 36
            isCurrent: ui.page === 1
            onClicked: {
                ui.page = 1;
            }
        }

        VerticalTab {
            id: logsTab

            icon.source: "qrc:/resources/assets/code.svg"
            text: qsTr("Network Logs")
            width: parent.width
            height: 36
            isCurrent: ui.page === 2
            onClicked: {
                ui.page = 2;
            }
        }

        VerticalTab {
            id: settingsTab

            icon.source: "qrc:/resources/assets/settings.svg"
            text: qsTr("Settings")
            width: parent.width
            height: 36
            isCurrent: ui.page === 3
            onClicked: {
                ui.page = 3;
            }
        }

    }

    VerticalTab {
        id: aboutTab

        anchors.bottom: parent.bottom
        anchors.right: parent.right
        anchors.left: parent.left
        anchors.margins: 4
        icon.source: "qrc:/resources/assets/info.svg"
        text: qsTr("About")
        width: parent.width
        height: 36
        isCurrent: ui.page === 4
        onClicked: {
            ui.page = 4;
        }
    }

}
