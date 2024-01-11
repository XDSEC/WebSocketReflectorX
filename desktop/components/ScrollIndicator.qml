import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.ScrollIndicator {
    id: control

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)

    contentItem: Rectangle {
        color: Style.palette.midlight
        implicitHeight: 4
        implicitWidth: 4
        radius: width > height ? height / 2 : width / 2
        opacity: control.active ? 1 : 0
        visible: control.size < 1

        Behavior on opacity {
            NumberAnimation {
                duration: 200
            }

        }

    }

}
