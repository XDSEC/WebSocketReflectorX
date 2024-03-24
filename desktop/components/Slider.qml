import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.Slider {
    id: control

    property bool useSystemFocusVisuals: true

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitHandleHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitHandleWidth + leftPadding + rightPadding)
    padding: 6

    background: Item {
        height: control.horizontal ? implicitHeight : control.availableHeight
        implicitHeight: control.horizontal ? 24 : 200
        implicitWidth: control.horizontal ? 200 : 24
        scale: control.horizontal && control.mirrored ? -1 : 1
        width: control.horizontal ? control.availableWidth : implicitWidth
        x: control.leftPadding + (control.horizontal ? 0 : (control.availableWidth - width) / 2)
        y: control.topPadding + (control.horizontal ? (control.availableHeight - height) / 2 : 0)

        Rectangle {
            color: enabled ? Style.palette.dark : Style.palette.mid
            height: !control.horizontal ? ((1 - control.position) * (parent.height - 16) - 2) : hovered || pressed ? 4 : 2 // SliderTrackThemeHeight
            radius: 2
            width: control.horizontal ? ((1 - control.position) * (parent.width - 16) - 2) : hovered || pressed ? 4 : 2 // SliderTrackThemeHeight
            x: control.horizontal ? (control.position * (parent.width - 16) + 18) : (parent.width - width) / 2
            y: control.horizontal ? (parent.height - height) / 2 : 0
        }

        Rectangle {
            color: enabled ? Style.primary : Style.palette.dark
            height: !control.horizontal ? (control.position * (parent.height - 16) - 2) : hovered || pressed ? 4 : 2 // SliderTrackThemeHeight
            radius: 2
            width: control.horizontal ? (control.position * (parent.width - 16) - 2) : hovered || pressed ? 4 : 2 // SliderTrackThemeHeight
            x: control.horizontal ? 0 : (parent.width - width) / 2
            y: control.horizontal ? (parent.height - height) / 2 : ((control.position * parent.height - 16) + 18)
        }

    }

    handle: Rectangle {
        border.color: enabled ? Style.primary : Style.palette.dark
        border.width: 4
        color: hovered || pressed ? (enabled ? Style.primary : Style.palette.dark) : "transparent"
        height: 16
        radius: 8
        width: 16
        x: control.leftPadding + (control.horizontal ? control.position * (control.availableWidth - width) : (control.availableWidth - width) / 2)
        y: control.topPadding + (control.horizontal ? (control.availableHeight - height) / 2 : control.visualPosition * (control.availableHeight - height))

        Behavior on color {
            ColorAnimation {
                duration: 200
            }

        }

    }

}
