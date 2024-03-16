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

    Label {
        id: logoTitle

        anchors.top: logoImage.bottom
        anchors.horizontalCenter: parent.horizontalCenter
        text: "WebSocket Reflector X"
        font.pixelSize: 18
        font.bold: true
    }

    Row {
        id: listenEdit
        width: addressEdit.width
        clip: true
        spacing: 8

        anchors.top: parent.verticalCenter
        anchors.topMargin: 32
        anchors.horizontalCenter: parent.horizontalCenter
        height: root.listenEditExtended ? 40 : 0

        Behavior on height {
            NumberAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.InOutExpo
            }
        }

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

            Timer {
                id: refreshTimer
                interval: 2400
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

            onClicked: {
                ui.refreshAvailableAddresses();
                refreshListenButton.icon.color = "transparent";
                refreshTimer.running = true;
                loadingTimer.running = true;
                loadingSpinner.running = true;
                loadingSpinner.opacity = 1;
                addressCombo.enabled = false;
            }
            
            ToolTip {
                parent: refreshListenButton
                visible: parent.hovered

                text: qsTr("Refresh available in-bound addresses and ports")
            }
        }

        ComboBox {
            id: addressCombo

            model: ui.availableAddresses
            width: 360
        }

        TextBox {
            id: portEdit
            width: 104
            height: 40
            placeholder: qsTr("Port")
            inputText: "0"
            inputArea.validator: IntValidator { bottom:0; top: 65535 }

            ToolTip {
                parent: portEdit
                visible: portEdit.state === "Focus"

                text: qsTr("Use 0 to get random available port.")
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

        TextBox {
            id: urlTextEdit
            width: 420
            height: 40
            placeholder: "[ws|wss]://..."
        }

        Button {
            display: AbstractButton.IconOnly
            height: 40
            icon.source: "qrc:/resources/assets/send.svg"
            icon.width: 20
            icon.height: 20
            borderWidth: 0
        }
    }
}
