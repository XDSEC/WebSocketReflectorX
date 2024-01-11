import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.Popup {
    id: control

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentWidth + leftPadding + rightPadding)
    padding: 12

    T.Overlay.modal: Rectangle {
        color: "transparent"
    }

    T.Overlay.modeless: Rectangle {
        color: "transparent"
    }

    background: Rectangle {
        border.color: Style.palette.mid
        border.width: 1
        color: Style.palette.toolTipBase
        radius: 8
    }

}
