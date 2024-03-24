import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.MenuSeparator {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: 4

    contentItem: Rectangle {
        implicitWidth: 188
        implicitHeight: 1
        color: Style.palette.mid
    }

    background: Rectangle {
        color: "transparent"
    }

}
