import QtQuick
import QtQuick.Controls
import QtQuick.Controls.impl
import QtQuick.Layouts
import Rx.Widgets

Item {
    id: root

    IconLabel {
        anchors.centerIn: parent
        text: qsTr("No permitted websites")
        icon.source: "qrc:/resources/assets/link-dismiss.svg"
        spacing: 16
        icon.color: Style.palette.buttonText
        color: Style.palette.buttonText
        display: AbstractButton.TextBesideIcon
        opacity: websites.size > 0 ? 0 : 0.6
    }

    ListView {
        anchors.fill: parent
        bottomMargin: 16
        leftMargin: 32
        rightMargin: 32
        topMargin: 16
        spacing: 8
        model: websites
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

        delegate: IconLabel {
            width: ListView.view.width - 64
            height: 40
            alignment: Qt.AlignLeft | Qt.AlignVCenter
            text: domain
            spacing: 8
            color: Style.palette.buttonText
            icon.source: "qrc:/resources/assets/shield.svg"
            icon.color: passed ? Style.palette.success : Style.palette.warning

            HoverHandler {
                id: hoverHandler
            }

            Rectangle {
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.right: parent.right
                height: 1
                color: Style.palette.midlight
            }
            
            Button {
                id: removeButton

                anchors.right: parent.right
                anchors.verticalCenter: parent.verticalCenter
                width: 40
                height: 40
                flat: true
                display: AbstractButton.IconOnly
                icon.source: "qrc:/resources/assets/delete.svg"
                icon.color: Style.palette.error
                opacity: hoverHandler.hovered ? 1 : 0
                onClicked: {
                    websites.deny(domain);
                }

                Behavior on opacity {
                    NumberAnimation {
                        duration: Style.midAnimationDuration
                    }

                }

            }

            Button {
                id: passButton

                anchors.right: removeButton.left
                anchors.verticalCenter: parent.verticalCenter
                width: 40
                height: 40
                flat: true
                display: AbstractButton.IconOnly
                icon.source: "qrc:/resources/assets/checkmark.svg"
                icon.color: Style.palette.success
                opacity: hoverHandler.hovered ? 1 : 0
                onClicked: {
                    websites.pass(domain);
                }

                Behavior on opacity {
                    NumberAnimation {
                        duration: Style.midAnimationDuration
                    }

                }

            }

        }

    }

}
