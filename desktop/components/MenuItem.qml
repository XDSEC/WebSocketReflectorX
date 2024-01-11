import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import Rx.Widgets

T.MenuItem {
    id: control

    icon.color: !enabled ? Color.transparent(Style.palette.text, 0.6) : Style.palette.text
    icon.height: 16
    icon.width: 16
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    leftPadding: 12
    padding: 4
    rightPadding: 12
    spacing: 12

    HoverHandler {
        cursorShape: Qt.PointingHandCursor
    }

    arrow: ColorImage {
        color: !enabled ? Color.transparent(Style.palette.text, 0.4) : Style.palette.text
        mirror: control.mirrored
        source: "qrc:/resources/assets/chevron-right.svg"
        visible: control.subMenu
        x: control.mirrored ? control.leftPadding : control.width - width - control.rightPadding
        y: control.topPadding + (control.availableHeight - height) / 2
    }

    background: Rectangle {
        color: !control.enabled ? "transparent" : control.down ? Style.palette.dark : control.highlighted ? Style.palette.mid : "transparent"
        implicitHeight: 36
        implicitWidth: 200
        radius: 6

        Rectangle {
            color: Style.palette.highlight
            height: parent.height - 2
            opacity: 0.5
            visible: control.visualFocus
            width: parent.width - 2
            x: 1
            y: 1
        }

        Behavior on color {
            ColorAnimation {
                duration: 120
            }

        }

    }

    contentItem: IconLabel {
        readonly property real arrowPadding: control.subMenu && control.arrow ? control.arrow.width + control.spacing : 0
        readonly property real indicatorPadding: control.checkable && control.indicator ? control.indicator.width + control.spacing : 0

        alignment: Qt.AlignLeft
        color: !control.enabled ? Color.transparent(Style.palette.text, 0.6) : Style.palette.text
        display: control.display
        font: control.font
        icon: control.icon
        leftPadding: !control.mirrored ? indicatorPadding : arrowPadding
        mirrored: control.mirrored
        rightPadding: control.mirrored ? indicatorPadding : arrowPadding
        spacing: control.spacing
        text: control.text
    }

    indicator: Rectangle {
        color: "transparent"
        visible: control.checkable
        height: control.checkable ? 16 : 0
        width: control.checkable ? 16 : 0
        radius: 6
        border.width: 1
        border.color: Color.transparent(Style.palette.text, 0.6)
        x: control.text ? (control.mirrored ? control.width - width - control.rightPadding : control.leftPadding) : control.leftPadding + (control.availableWidth - width) / 2
        y: control.topPadding + (control.availableHeight - height) / 2

        ColorImage {
            color: !control.enabled ? Color.transparent(Style.palette.text, 0.4) : Style.palette.text
            source: !control.checkable ? "" : "qrc:/resources/assets/checkmark-filled.svg"
            visible: control.checked
            width: 14
            height: 14
            anchors.centerIn: parent
        }

    }

}
