import QtQuick
import QtQuick.Templates as T
import QtQuick.Window
import Rx.Widgets

T.Menu {
    id: control

    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, contentHeight + topPadding + bottomPadding)
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentWidth + leftPadding + rightPadding)
    margins: 0
    overlap: 4
    padding: 4

    enter: Transition {
        NumberAnimation {
            property: "opacity"
            from: 0
            to: 1
            duration: 120
        }

        NumberAnimation {
            property: "height"
            from: implicitHeight / 2
            to: implicitHeight
            duration: 300
            easing.type: Easing.OutExpo
        }

    }

    exit: Transition {
        NumberAnimation {
            property: "opacity"
            from: 1
            to: 0
            duration: 120
        }

        NumberAnimation {
            property: "height"
            from: implicitHeight
            to: implicitHeight / 2
            duration: 300
            easing.type: Easing.OutExpo
        }

    }

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
        implicitHeight: 42
        implicitWidth: 200
        radius: 8
    }

    contentItem: ListView {
        clip: true
        currentIndex: control.currentIndex
        implicitHeight: contentHeight
        interactive: Window.window ? contentHeight + control.topPadding + control.bottomPadding > Window.window.height : false
        model: control.contentModel

        ScrollIndicator.vertical: ScrollIndicator {
        }

    }

    delegate: MenuItem {
    }

}
