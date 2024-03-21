import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Rx.Widgets

ScrollView {
    id: root

    clip: true
    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

    Image {
        id: logoImage

        anchors.left: parent.left
        anchors.leftMargin: 32
        anchors.top: parent.top
        anchors.topMargin: 16
        source: "qrc:/resources/assets/logo.svg"
        sourceSize: Qt.size(96, 96)
        width: 96
        height: 96
    }

    Label {
        id: logoText

        text: "Web Socket Reflector X"
        anchors.left: logoImage.right
        anchors.leftMargin: 16
        anchors.right: parent.right
        anchors.rightMargin: 32
        anchors.top: logoImage.top
        anchors.topMargin: 8
        font.pixelSize: 24
        opacity: 0.8
    }

    Label {
        id: sloganText

        anchors.left: logoText.left
        anchors.top: logoText.bottom
        anchors.right: logoText.right
        anchors.topMargin: 8
        opacity: 0.6
        text: "Idealism is that you will probably never receive something back,\nbut nonetheless still decide to give."
    }

    Switch {
        id: runningInTraySwitch

        anchors.top: sloganText.bottom
        anchors.topMargin: 64
        anchors.left: logoImage.left
        anchors.right: logoText.right
        text: qsTr("Running in system tray when closed")
    }

    Rectangle {
        id: separator

        anchors.top: runningInTraySwitch.bottom
        anchors.topMargin: 16
        anchors.left: logoImage.left
        anchors.right: logoText.right
        height: 1
        color: Style.palette.alternateBase
    }

    Label {
        id: debugInfoTitleText

        anchors.top: separator.bottom
        anchors.topMargin: 24
        anchors.left: logoImage.left
        anchors.right: logoText.right
        text: qsTr("System information for bug reporting and debugging")
    }

    Label {
        id: debugInfoTipsText

        anchors.top: debugInfoTitleText.bottom
        anchors.topMargin: 8
        anchors.left: debugInfoTitleText.left
        anchors.right: logoText.right
        text: qsTr("Please include the following information when reporting bugs or asking for help.")
        opacity: 0.6
    }

    ScrollView {
        id: debugInfoArea

        anchors.top: debugInfoTipsText.bottom
        anchors.topMargin: 8
        anchors.left: debugInfoTitleText.left
        anchors.right: logoText.right
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

        TextArea {
            id: innerDebugInfoArea

            readOnly: true
            text: daemon.systemInfo
        }

    }

    Button {
        id: copyButton

        anchors.top: debugInfoArea.top
        anchors.topMargin: 8
        anchors.right: debugInfoArea.right
        anchors.rightMargin: 8
        display: AbstractButton.TextBesideIcon
        icon.source: "qrc:/resources/assets/copy.svg"
        icon.color: Style.palette.success
        text: qsTr("Copy")
        onClicked: {
            innerDebugInfoArea.selectAll();
            innerDebugInfoArea.copy();
            innerDebugInfoArea.deselect();
            icon.source = "qrc:/resources/assets/checkmark.svg";
            text = qsTr("Copied!");
            copyButtonTimer.start();
        }

        Timer {
            id: copyButtonTimer

            interval: 1000
            repeat: false
            onTriggered: {
                copyButton.icon.source = "qrc:/resources/assets/copy.svg";
                copyButton.text = qsTr("Copy");
            }
        }

    }

    Label {
        id: copyrightText

        anchors.left: logoImage.left
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 16
        anchors.right: logoText.right
        text: qsTr("(c) 2022 - 2024 Ret 2 Shell, distributed with MIT license. Source code available at <a href=\"https://github.com/ret2shell/wsrx\">here</a>.")
        onLinkActivated: (link) => {
            return Qt.openUrlExternally(link);
        }
        opacity: 0.6
    }

}
