import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.Pane {
    id: control

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentWidth + leftPadding + rightPadding)
    padding: 12

    background: Rectangle {
        color: "transparent"
    }

}
