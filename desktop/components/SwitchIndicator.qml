import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

Item {
    id: indicator

    property T.AbstractButton control

    implicitWidth: 40
    implicitHeight: 20

    Rectangle {
        width: parent.width - 4
        height: 4
        radius: 2
        color: indicator.control.checked ? Style.palette.primary : Style.palette.dark
        x: 2
        y: (parent.height - height) / 2

        Behavior on color {
            enabled: !indicator.control.pressed

            ColorAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.InOutQuad
            }

        }

    }

    Rectangle {
        width: 20
        height: 20
        radius: 10
        border.width: 2
        border.color: Style.palette.dark
        color: indicator.control.checked ? Style.palette.primary : Style.palette.debug
        x: Math.max(0, Math.min(parent.width - width, indicator.control.visualPosition * parent.width - (width / 2)))
        y: (parent.height - height) / 2

        Behavior on x {
            enabled: !indicator.control.pressed

            NumberAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.InOutQuad
            }

        }

        Behavior on color {
            enabled: !indicator.control.pressed

            ColorAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.InOutQuad
            }

        }

    }

}
