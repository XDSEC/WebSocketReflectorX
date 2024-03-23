import QtQuick
import QtQuick.Controls
import QtQuick.Controls.impl
import QtQuick.Layouts
import Rx.Widgets

Item {
    id: root

    IconLabel {
        anchors.centerIn: parent
        text: qsTr("No logs")
        icon.source: "qrc:/resources/assets/code.svg"
        spacing: 16
        icon.color: Style.palette.buttonText
        color: Style.palette.buttonText
        display: AbstractButton.TextBesideIcon
        opacity: logs.size > 0 ? 0 : 0.6
    }

    ListView {
        anchors.fill: parent
        bottomMargin: 16
        leftMargin: 32
        rightMargin: 32
        topMargin: 16
        model: logs
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

        delegate: Item {
            width: ListView.view.width - 64
            height: logsEdit.implicitHeight + 16

            IconLabel {
                id: levelIndicator

                alignment: Qt.AlignVCenter | Qt.AlignHCenter
                display: AbstractButton.IconOnly
                anchors.top: parent.top
                anchors.topMargin: 10
                anchors.left: parent.left
                anchors.leftMargin: 8
                icon.color: {
                    switch (level) {
                    case 0:
                        return Style.palette.info;
                    case 1:
                        return Style.palette.warning;
                    case 2:
                        return Style.palette.error;
                    case 3:
                        return Style.palette.success;
                    default:
                        return Style.palette.text;
                    }
                }
                icon.source: {
                    switch (level) {
                    case 0:
                        return "qrc:/resources/assets/info.svg";
                    case 1:
                        return "qrc:/resources/assets/warning.svg";
                    case 2:
                        return "qrc:/resources/assets/error-circle.svg";
                    case 3:
                        return "qrc:/resources/assets/checkmark-circle.svg";
                    default:
                        return "qrc:/resources/assets/info.svg";
                    }
                }
            }

            HoverHandler {
                id: logHoverHandler
            }

            ToolTip {
                text: timestamp
                visible: logHoverHandler.hovered
                y: 2
                x: levelIndicator.width + logsEdit.width - width
            }

            TextEdit {
                id: logsEdit

                readOnly: true
                text: `[${target}] ${message}`
                anchors.left: levelIndicator.right
                anchors.right: parent.right
                anchors.leftMargin: 8
                anchors.verticalCenter: parent.verticalCenter
                horizontalAlignment: TextEdit.AlignLeft
                verticalAlignment: TextEdit.AlignVCenter
                wrapMode: TextEdit.Wrap
                // font.bold: root.isActive
                color: Style.palette.text
                selectByMouse: true
                selectedTextColor: Style.palette.text
                selectionColor: Color.transparent(Style.palette.primary, 0.4)
                opacity: {
                    switch (level) {
                    case 0:
                        return 0.6;
                    case 1:
                        return 0.9;
                    case 2:
                        return 1;
                    case 3:
                        return 0.8;
                    default:
                        return 0.6;
                    }
                }
            }

            Rectangle {
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.right: parent.right
                height: 1
                color: Style.palette.midlight
            }

        }

    }

}
