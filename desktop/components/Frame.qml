import QtQuick
import QtQuick.Controls
import QtQuick.Templates as T
import Rx.Widgets

T.Frame {
    id: control

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentWidth + leftPadding + rightPadding)
    padding: 12

    background: Rectangle {
        implicitHeight: 100
        implicitWidth: 100
    }

}
