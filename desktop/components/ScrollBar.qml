import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.ScrollBar {
    id: control

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    minimumSize: 0.1
    visible: control.policy !== T.ScrollBar.AlwaysOff
    states: [
        State {
            name: "active"
            when: control.policy === T.ScrollBar.AlwaysOn || (control.active && control.size < 1)
        }
    ]
    transitions: [
        Transition {
            to: "active"

            NumberAnimation {
                property: "opacity"
                targets: [control.contentItem, control.background]
                to: 1
            }

        },
        Transition {
            from: "active"

            SequentialAnimation {
                PropertyAction {
                    property: "opacity"
                    targets: [control.contentItem, control.background]
                    value: 1
                }

                PauseAnimation {
                    duration: 3000
                }

                NumberAnimation {
                    property: "opacity"
                    targets: [control.contentItem, control.background]
                    to: 0
                }

            }

        }
    ]

    background: Rectangle {
        color: "transparent"
        implicitHeight: 12
        implicitWidth: 12
        opacity: 0
        visible: control.size < 1
    }

    contentItem: Rectangle {
        color: "transparent"
        implicitHeight: 12
        implicitWidth: 12
        opacity: 0

        Rectangle {
            anchors.fill: parent
            anchors.margins: control.pressed || control.hovered ? 3 : 4
            color: Style.palette.buttonText
            radius: width > height ? height / 2 : width / 2
            opacity: control.pressed ? 1 : enabled && control.interactive && control.hovered ? 0.8 : 0.6

            Behavior on anchors.margins {
                NumberAnimation {
                    duration: Style.shortAnimationDuration
                }

            }

        }

    }

}
