import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import QtQuick.Window
import Rx.Widgets

T.ComboBox {
    id: control

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    padding: 6
    leftPadding: padding * 2 + (!control.mirrored || !indicator || !indicator.visible ? 0 : indicator.width + spacing)
    rightPadding: padding * 2 + (control.mirrored || !indicator || !indicator.visible ? 0 : indicator.width + spacing)

    HoverHandler {
        cursorShape: Qt.PointingHandCursor
    }

    background: Rectangle {
        border.color: "transparent"
        border.width: control.flat ? 0 : 1 // ComboBoxBorderThemeThickness
        color: control.enabled ? (control.hovered ? Style.palette.midlight : Style.palette.button) : Style.palette.dark
        implicitHeight: 40
        implicitWidth: 120
        radius: 6
        visible: !control.flat || control.pressed || control.hovered || control.visualFocus
    }

    contentItem: T.TextField {
        autoScroll: control.editable
        color: control.enabled ? Style.palette.buttonText : Style.palette.placeholderText
        enabled: control.editable
        inputMethodHints: control.inputMethodHints
        leftPadding: control.mirrored ? 1 : 4
        readOnly: control.down
        rightPadding: control.mirrored ? 4 : 1
        selectByMouse: control.selectTextByMouse
        selectedTextColor: Style.palette.highlightedText
        selectionColor: Style.palette.highlight
        text: control.editable ? control.editText : control.displayText
        validator: control.validator
        verticalAlignment: Text.AlignVCenter
    }

    delegate: ItemDelegate {
        required property var model
        required property int index

        hoverEnabled: control.hoverEnabled
        implicitHeight: 36
        text: model[control.textRole]
        font.weight: control.currentIndex === index ? Font.DemiBold : Font.Normal
        width: ListView.view.width

        HoverHandler {
            cursorShape: Qt.PointingHandCursor
        }

        background: Rectangle {
            color: hovered ? Style.palette.mid : "transparent"
            implicitHeight: 36
            implicitWidth: 200
            radius: 6

            Behavior on color {
                ColorAnimation {
                    duration: Style.shortAnimationDuration
                }

            }

        }

    }

    indicator: ColorImage {
        color: control.enabled ? Style.palette.buttonText : Style.palette.mid
        source: "qrc:/resources/assets/chevron-down.svg"
        x: control.mirrored ? control.padding : control.width - width - control.padding * 2
        y: control.topPadding + (control.availableHeight - height) / 2
    }

    popup: T.Popup {
        bottomMargin: 8
        height: Math.min(contentItem.implicitHeight + padding * 2, control.Window.height - topMargin - bottomMargin)
        implicitHeight: Math.min(contentItem.implicitHeight + padding * 2, control.Window.height - topMargin - bottomMargin)
        topMargin: 8
        width: control.width
        y: control.height + 2
        padding: 4

        background: Rectangle {
            anchors.fill: parent
            border.color: Style.palette.mid
            border.width: 1
            color: Style.palette.toolTipBase
            radius: 8
        }

        contentItem: ListView {
            clip: true
            currentIndex: control.currentIndex
            implicitHeight: contentHeight
            interactive: Window.window ? contentHeight + control.topPadding + control.bottomPadding > Window.window.height : false
            model: control.delegateModel

            ScrollIndicator.vertical: ScrollIndicator {
            }

        }

        enter: Transition {
            NumberAnimation {
                duration: Style.shortAnimationDuration
                from: 0
                property: "opacity"
                to: 1
            }

            NumberAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.OutExpo
                from: control.popup.implicitHeight / 2
                property: "height"
                to: control.popup.implicitHeight
            }

        }

        exit: Transition {
            NumberAnimation {
                duration: Style.shortAnimationDuration
                from: 1
                property: "opacity"
                to: 0
            }

            NumberAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.OutExpo
                from: control.popup.implicitHeight
                property: "height"
                to: control.popup.implicitHeight / 2
            }

        }

    }

}
