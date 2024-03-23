import QtQuick
import QtQuick.Controls
import QtQuick.Controls.impl
import QtQuick.Templates as T
import Rx.Widgets

T.Button {
    id: control

    property int alignment: Qt.AlignVCenter | Qt.AlignHCenter
    property color hoverColor: Style.palette.mid
    property color normalColor: Style.palette.button
    property color pressedColor: Style.palette.dark
    property color activeColor: Style.palette.midlight
    property int radius: 6
    property bool square: false
    property int borderWidth: 0
    property bool active: false

    display: AbstractButton.TextOnly
    flat: false
    icon.color: active ? Style.palette.primary : Style.palette.buttonText
    icon.height: 16
    icon.width: 16
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    implicitWidth: square ? implicitHeight : Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    padding: 4
    leftPadding: square ? 4 : 12
    rightPadding: square ? 4 : 12
    spacing: 12

    HoverHandler {
        cursorShape: Qt.PointingHandCursor
    }

    background: Rectangle {
        border.color: Style.palette.mid
        border.width: flat ? 0 : control.borderWidth
        color: {
            if (!enabled)
                return Style.palette.midlight;

            if (pressed)
                return control.pressedColor;
            else if (hovered)
                return control.hoverColor;
            else if (active)
                return control.activeColor;
            else if (flat)
                return "transparent";
            else
                return control.normalColor;
        }
        implicitHeight: 40
        implicitWidth: 40
        radius: control.radius

        Behavior on color {
            ColorAnimation {
                duration: 120
            }

        }

    }

    contentItem: IconLabel {
        alignment: control.alignment
        color: Color.transparent(Style.palette.buttonText, enabled ? 1 : 0.2)
        display: control.display
        font: control.font
        icon: control.icon
        mirrored: control.mirrored
        spacing: control.spacing
        text: control.text
        opacity: control.enabled ? 1 : 0.6
    }

}
