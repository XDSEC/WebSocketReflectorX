import QtQuick
import QtQuick.Controls
import QtQuick.Controls.impl
import QtQuick.Layouts
import Rx.Widgets

Item {
    id: root

    IconLabel {
        anchors.centerIn: parent
        text: qsTr("No connections")
        icon.source: "qrc:/resources/assets/link-dismiss.svg"
        spacing: 16
        icon.color: Style.palette.buttonText
        color: Style.palette.buttonText
        display: AbstractButton.TextBesideIcon
        opacity: links.size > 0 ? 0 : 0.6
    }

    ListView {
        anchors.fill: parent
        bottomMargin: 16
        leftMargin: 32
        rightMargin: 32
        topMargin: 16
        spacing: 8
        model: links
        clip: true

        add: Transition {
            NumberAnimation {
                properties: "x"
                from: 100
                duration: Style.midAnimationDuration
                easing.type: Easing.OutExpo
            }

            NumberAnimation {
                properties: "opacity"
                from: 0
                to: 1
                duration: Style.midAnimationDuration
            }

        }

        addDisplaced: Transition {
            NumberAnimation {
                properties: "y"
                duration: Style.midAnimationDuration
                easing.type: Easing.OutExpo
            }

        }

        removeDisplaced: Transition {
            PauseAnimation {
                duration: 300
            }

            NumberAnimation {
                properties: "y"
                duration: Style.midAnimationDuration
            }

        }

        remove: Transition {
            NumberAnimation {
                properties: "x"
                to: 100
                duration: Style.midAnimationDuration
                easing.type: Easing.OutExpo
            }

            NumberAnimation {
                properties: "opacity"
                from: 1
                to: 0
                duration: Style.midAnimationDuration
                easing.type: Easing.OutExpo
            }

        }

        ScrollBar.vertical: ScrollBar {
        }

        delegate: GridLayout {
            width: ListView.view.width - 64
            columns: 5

            IconLabel {
                Layout.leftMargin: 16
                Layout.rightMargin: 16
                icon.color: {
                    switch (status) {
                    case 0:
                        return Style.palette.info;
                    case 1:
                        return Style.palette.success;
                    case 2:
                        return Style.palette.error;
                    }
                }
                icon.source: {
                    switch (status) {
                    case 0:
                        return "qrc:/resources/assets/link.svg";
                    case 1:
                        return "qrc:/resources/assets/plug-connected.svg";
                    case 2:
                        return "qrc:/resources/assets/plug-disconnected.svg";
                    }
                }
                display: AbstractButton.IconOnly
                Layout.rowSpan: 2
            }

            TextEdit {
                id: fromEdit

                text: from
                readOnly: true
                horizontalAlignment: TextEdit.AlignLeft
                verticalAlignment: TextEdit.AlignVCenter
                wrapMode: TextEdit.Wrap
                font.bold: true
                color: Style.palette.text
                selectByMouse: true
                selectedTextColor: Style.palette.text
                selectionColor: Color.transparent(Style.palette.primary, 0.4)
            }

            IconLabel {
                Layout.leftMargin: 16
                Layout.rightMargin: 16
                text: `${status === 1 ? latency:'--'} ms`
                Layout.fillWidth: true
                alignment: Qt.AlignLeft | Qt.AlignVCenter
                color: status === 2 ? Style.palette.error : (latency > 100 ? (latency > 200 ? Style.palette.error : Style.palette.warning) : Style.palette.success)
            }

            Button {
                id: copyButton

                Layout.leftMargin: 4
                Layout.rightMargin: 4
                display: AbstractButton.IconOnly
                icon.source: "qrc:/resources/assets/copy.svg"
                icon.color: Style.palette.success
                Layout.rowSpan: 2
                flat: true
                opacity: hoverHandler.hovered ? 1 : 0
                onClicked: {
                    fromEdit.selectAll();
                    fromEdit.copy();
                    fromEdit.deselect();
                    copyButton.icon.source = "qrc:/resources/assets/checkmark.svg";
                    timer.start();
                }

                Behavior on opacity {
                    NumberAnimation {
                        duration: Style.midAnimationDuration
                    }
                }

                Timer {
                    id: timer

                    interval: 1000
                    running: false
                    repeat: false
                    onTriggered: {
                        copyButton.icon.source = "qrc:/resources/assets/copy.svg";
                    }
                }

            }

            Button {
                Layout.leftMargin: 4
                Layout.rightMargin: 8
                display: AbstractButton.IconOnly
                icon.source: "qrc:/resources/assets/delete.svg"
                icon.color: Style.palette.error
                Layout.rowSpan: 2
                flat: true
                opacity: hoverHandler.hovered ? 1 : 0
                onClicked: {
                    daemon.requestDisconnect(from);
                }

                Behavior on opacity {
                    NumberAnimation {
                        duration: Style.midAnimationDuration
                    }
                }
            }

            Label {
                text: to
                Layout.columnSpan: 2
                opacity: 0.6
            }

            Rectangle {
                Layout.columnSpan: 5
                height: 1
                width: parent.width
                color: Style.palette.midlight
            }

            HoverHandler {
                id: hoverHandler
            }

        }

    }

}
