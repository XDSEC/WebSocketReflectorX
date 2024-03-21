import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.Switch {
    id: control

    implicitWidth: implicitContentWidth
    implicitHeight: implicitContentHeight

    property bool useSystemFocusVisuals: true

    indicator: SwitchIndicator {
        anchors.verticalCenter: parent.verticalCenter
        anchors.left: control.mirrored ? parent.left : undefined
        anchors.right: !control.mirrored ? parent.right : undefined
        anchors.leftMargin: control.mirrored ? control.padding : 0
        anchors.rightMargin: !control.mirrored ? control.padding : 0
        control: control
    }

    HoverHandler {
        cursorShape: Qt.PointingHandCursor
    }

    contentItem: Text {
        leftPadding: 0
        rightPadding: 0

        text: control.text
        font: control.font
        elide: Text.ElideRight
        verticalAlignment: Text.AlignVCenter

        opacity: enabled ? 1.0 : 0.2
        color: Style.palette.buttonText
    }
}
