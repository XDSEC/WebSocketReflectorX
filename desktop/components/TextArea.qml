import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import Rx.Widgets

T.TextArea {
    id: control

    implicitWidth: Math.max(contentWidth + leftPadding + rightPadding, implicitBackgroundWidth + leftInset + rightInset, placeholder.implicitWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(contentHeight + topPadding + bottomPadding, implicitBackgroundHeight + topInset + bottomInset, placeholder.implicitHeight + topPadding + bottomPadding)
    padding: 12
    topInset: 0
    bottomInset: 0
    color: Style.palette.text
    selectionColor: Color.transparent(Style.palette.primary, 0.4)
    selectedTextColor: Style.palette.text
    placeholderTextColor: Style.palette.placeholderText

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
