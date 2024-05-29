import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Rx.Widgets

Item {
    id: root

    property bool listenEditExtended: false

    Image {
        id: logoImage

        anchors.bottom: parent.verticalCenter
        anchors.bottomMargin: 64
        anchors.horizontalCenter: parent.horizontalCenter
        source: "qrc:/resources/assets/logo.svg"
        sourceSize: Qt.size(160, 160)
    }

    Button {
        id: updatesButton
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        anchors.margins: 6
        visible: ui.hasNewVersion
        display: AbstractButton.TextBesideIcon
        text: qsTr("Updates Available")
        icon.source: "qrc:/resources/assets/wand.svg"
        icon.color: Style.palette.warning
        onClicked: {
            Qt.openUrlExternally(ui.updateUrl);
        }
    }

    Label {
        id: logoTitle

        anchors.top: logoImage.bottom
        anchors.horizontalCenter: parent.horizontalCenter
        text: "WebSocket Reflector X"
        font.pixelSize: 24
        font.bold: true
    }

    Row {
        id: listenEdit

        width: addressEdit.width
        clip: true
        spacing: 8
        anchors.top: parent.verticalCenter
        anchors.topMargin: 64
        anchors.horizontalCenter: parent.horizontalCenter
        height: root.listenEditExtended ? 40 : 0

        Button {
            id: refreshListenButton

            display: AbstractButton.IconOnly
            height: 40
            icon.source: "qrc:/resources/assets/arrow-clockwise.svg"
            icon.width: 20
            icon.height: 20
            icon.color: Style.palette.buttonText
            borderWidth: 0
            hoverEnabled: true
            onClicked: {
                if (refreshTimer.running)
                    return ;

                daemon.refreshAvailableAddresses();
                refreshListenButton.icon.color = Style.palette.button;
                refreshTimer.running = true;
                loadingTimer.running = true;
                loadingSpinner.running = true;
                loadingSpinner.opacity = 1;
                addressCombo.enabled = false;
            }

            Timer {
                id: refreshTimer

                interval: 1700
                running: false
                repeat: false
                onTriggered: {
                    refreshListenButton.icon.color = Style.palette.buttonText;
                    refreshListenButton.icon.source = "qrc:/resources/assets/arrow-clockwise.svg";
                }
            }

            Timer {
                id: loadingTimer

                interval: 1000
                running: false
                repeat: false
                onTriggered: {
                    refreshListenButton.icon.color = Style.palette.success;
                    refreshListenButton.icon.source = "qrc:/resources/assets/checkmark.svg";
                    loadingSpinner.running = false;
                    loadingSpinner.opacity = 0;
                    addressCombo.enabled = true;
                }
            }

            Loading {
                id: loadingSpinner

                anchors.centerIn: parent
                radius: 8
                running: false
                opacity: 0
            }

            ToolTip {
                parent: refreshListenButton
                visible: parent.hovered
                text: qsTr("Refresh available in-bound addresses and ports")
            }

        }

        ComboBox {
            id: addressCombo

            model: daemon.availableAddresses
            width: 360
        }

        TextField {
            id: portEdit

            width: 104
            height: 40
            placeholderText: qsTr("Port")
            text: "0"
            hoverEnabled: true

            ToolTip {
                parent: portEdit
                visible: portEdit.hovered
                text: qsTr("Use 0 to get random available port.")
            }

            validator: IntValidator {
                bottom: 0
                top: 65535
            }

        }

        Behavior on height {
            NumberAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.InOutExpo
            }

        }

    }

    Row {
        id: addressEdit

        anchors.top: listenEdit.bottom
        anchors.topMargin: 8
        anchors.horizontalCenter: parent.horizontalCenter
        spacing: 8

        Button {
            id: inboundButton

            display: AbstractButton.IconOnly
            height: 40
            icon.source: "qrc:/resources/assets/settings.svg"
            icon.width: 20
            icon.height: 20
            borderWidth: 0
            active: root.listenEditExtended
            onClicked: {
                root.listenEditExtended = !root.listenEditExtended;
            }
            hoverEnabled: true

            ToolTip {
                parent: inboundButton
                visible: parent.hovered
                text: qsTr("Configure In-bound Address")
            }

        }

        TextField {
            id: urlTextEdit

            width: 420
            height: 40
            placeholderText: "[ws|wss]://..."

            Menu {
                id: contentMenu

                MenuItem {
                    icon.source: "qrc:/resources/assets/add-square-multiple.svg"
                    text: qsTr("Select All")
                    onTriggered: urlTextEdit.selectAll()
                }

                MenuItem {
                    icon.source: "qrc:/resources/assets/cut.svg"
                    text: qsTr("Cut")
                    onTriggered: urlTextEdit.cut()
                }

                MenuItem {
                    icon.source: "qrc:/resources/assets/copy.svg"
                    text: qsTr("Copy")
                    onTriggered: urlTextEdit.copy()
                }

                MenuItem {
                    icon.source: "qrc:/resources/assets/clipboard-paste.svg"
                    text: qsTr("Paste")
                    onTriggered: urlTextEdit.paste()
                }

            }

            TapHandler {
                acceptedButtons: Qt.RightButton
                onTapped: {
                    contentMenu.popup();
                }
            }

            ToolTip {
                id: errorTip

                parent: urlTextEdit
                timeout: 5000
                visible: false
                contentColor: Style.palette.error
                y: parent.height + 8
            }

        }

        Button {
            id: connectButton

            display: AbstractButton.IconOnly
            height: 40
            icon.source: "qrc:/resources/assets/send.svg"
            icon.width: 20
            icon.height: 20
            borderWidth: 0
            onClicked: {
                if (connectTimer.running)
                    return ;

                daemon.requestConnect(urlTextEdit.text, addressCombo.currentText, portEdit.text);
                connectButton.icon.color = Style.palette.button;
                connectTimer.running = true;
                connectingSpinner.running = true;
                connectingSpinner.opacity = 1;
            }

            Loading {
                id: connectingSpinner

                anchors.centerIn: parent
                radius: 8
                running: false
                opacity: 0
            }

        }

        Timer {
            id: connectTimer

            interval: 1000
            running: false
            repeat: false
            onTriggered: {
                connectButton.icon.color = Style.palette.buttonText;
                connectButton.icon.source = "qrc:/resources/assets/send.svg";
                connectingSpinner.running = false;
                connectingSpinner.opacity = 0;
            }
        }

        Connections {
            function onConnected(success, message) {
                if (!success) {
                    errorTip.text = message;
                    errorTip.visible = true;
                } else {
                    errorTip.visible = false;
                }
            }

            target: daemon
        }

    }

}
