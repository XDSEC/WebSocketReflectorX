import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import Rx.Widgets

T.SplitView {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)

    handle: Rectangle {
        implicitWidth: control.orientation === Qt.Horizontal ? 1 : control.width
        implicitHeight: control.orientation === Qt.Horizontal ? control.height : 1
        color: T.SplitHandle.pressed ? Style.palette.primary : (enabled && T.SplitHandle.hovered ? Style.palette.primary : Style.palette.midlight)

        Behavior on color {
            ColorAnimation {
                duration: Style.shortAnimationDuration
            }

        }

        containmentMask: Item {
            x: control.orientation === Qt.Horizontal ? (1 - width) / 2 : 0
            y: control.orientation === Qt.Vertical ? (1 - height) / 2 : 0
            width: control.orientation === Qt.Horizontal ? 12 : control.width
            height: control.orientation === Qt.Vertical ? 12 : control.height
        }

    }

}
