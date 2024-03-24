import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import Rx.Widgets

T.ScrollView {
    id: control

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentWidth + leftPadding + rightPadding)

    ScrollBar.horizontal: ScrollBar {
        active: control.ScrollBar.vertical.active
        parent: control
        width: control.availableWidth
        x: control.leftPadding
        y: control.height - height
    }

    ScrollBar.vertical: ScrollBar {
        active: control.ScrollBar.horizontal.active
        height: control.availableHeight
        parent: control
        x: control.mirrored ? 0 : control.width - width
        y: control.topPadding
    }

}
