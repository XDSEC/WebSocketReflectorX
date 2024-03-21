import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import Rx.Widgets

T.TextField {
    id: control

    implicitWidth: implicitBackgroundWidth + leftInset + rightInset || Math.max(contentWidth, placeholder.implicitWidth) + leftPadding + rightPadding
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding, placeholder.implicitHeight + topPadding + bottomPadding)
    padding: 12
    topPadding: 6
    bottomPadding: 6
    topInset: 0
    bottomInset: 0
    color: Style.palette.text
    selectedTextColor: Style.palette.text
    selectionColor: Color.transparent(Style.palette.primary, 0.4)
    placeholderTextColor: Style.palette.placeholderText
    verticalAlignment: TextInput.AlignVCenter

    PlaceholderText {
        id: placeholder

        x: control.leftPadding
        y: control.topPadding
        width: control.width - (control.leftPadding + control.rightPadding)
        height: control.height - (control.topPadding + control.bottomPadding)
        text: control.placeholderText
        font: control.font
        color: control.placeholderTextColor
        visible: !control.length && !control.preeditText && (!control.activeFocus || control.horizontalAlignment !== Qt.AlignHCenter)
        verticalAlignment: control.verticalAlignment
        elide: Text.ElideRight
        renderType: control.renderType
    }

    background: Rectangle {
        implicitWidth: 120
        implicitHeight: 36
        border.width: 2
        radius: 6
        border.color: !control.enabled ? Style.palette.mid : control.activeFocus ? Style.palette.midlight : control.hovered ? Style.palette.midlight : "transparent"
        color: control.enabled ? (control.activeFocus ? "transparent" : Style.palette.button) : Style.palette.mid
    }

    cursorDelegate: Rectangle {
        id: cursorDelegate

        color: Color.transparent(Style.palette.buttonText, 0.6)
        width: 1.5

        Connections {
            function onActiveFocusChanged() {
                if (target.activeFocus)
                    cursorAnimation.start();
                else
                    cursorAnimation.stop();
            }

            function onCursorPositionChanged() {
                if (target.activeFocus)
                    cursorAnimation.restart();

            }

            target: control
        }

        SequentialAnimation {
            id: cursorAnimation

            loops: SequentialAnimation.Infinite
            running: false
            onStarted: {
                cursorDelegate.visible = true;
            }
            onStopped: {
                cursorDelegate.visible = false;
            }

            NumberAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.OutCurve
                from: 1
                property: "opacity"
                target: cursorDelegate
                to: 0
            }

            PauseAnimation {
                duration: Style.midAnimationDuration
            }

            NumberAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.InCurve
                from: 0
                property: "opacity"
                target: cursorDelegate
                to: 1
            }

            PauseAnimation {
                duration: Style.midAnimationDuration
            }

        }

    }

}
