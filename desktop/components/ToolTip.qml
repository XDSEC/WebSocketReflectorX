import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.ToolTip {
    id: control

    property color color: Style.palette.toolTipBase
    property color contentColor: Style.palette.toolTipText

    bottomPadding: padding - 1
    closePolicy: T.Popup.CloseOnEscape | T.Popup.CloseOnPressOutsideParent | T.Popup.CloseOnReleaseOutsideParent
    delay: Qt.styleHints.mousePressAndHoldInterval
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentWidth + leftPadding + rightPadding)
    margins: 8
    padding: 6
    leftPadding: 12
    rightPadding: 12
    x: parent ? (parent.width - implicitWidth) / 2 : 0
    y: parent ? parent.height + 2 : implicitHeight + 2

    background: Rectangle {
        border.color: Style.palette.mid
        border.width: 1
        color: control.color
        radius: 6
    }

    contentItem: Text {
        color: control.contentColor
        font: control.font
        opacity: enabled ? 1 : 0.2
        text: control.text
        wrapMode: Text.Wrap
    }

    enter: Transition {
        NumberAnimation {
            property: "opacity"
            from: 0
            to: 1
            duration: 120
        }

        NumberAnimation {
            property: "height"
            from: implicitHeight / 2
            to: implicitHeight
            duration: 300
            easing.type: Easing.OutExpo
        }

    }

    exit: Transition {
        NumberAnimation {
            property: "opacity"
            from: 1
            to: 0
            duration: 120
        }

        NumberAnimation {
            property: "height"
            from: implicitHeight
            to: implicitHeight / 2
            duration: 300
            easing.type: Easing.OutExpo
        }

    }

}
