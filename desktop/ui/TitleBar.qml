import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Rx.Widgets

Rectangle {
    id: titleBar

    color: "transparent"
    height: 36

    Button {
        id: closeButton

        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        display: AbstractButton.IconOnly
        flat: true
        hoverColor: "#A0FF0000"
        icon.source: "qrc:/resources/assets/dismiss.svg"
        pressedColor: "#A0FF0000"
        icon.color: hovered ? "#FFFFFF" : Style.palette.buttonText
        radius: 0
        width: 48
        height: parent.height
        onClicked: {
            if (ui.runningInTray)
                window.close();
            else
                ui.requestToQuit();
        }
    }

    Button {
        id: maximizeButton

        anchors.right: closeButton.left
        anchors.verticalCenter: parent.verticalCenter
        display: AbstractButton.IconOnly
        flat: true
        icon.source: window.visibility === Window.Windowed ? "qrc:/resources/assets/square.svg" : "qrc:/resources/assets/square-multiple.svg"
        radius: 0
        width: 48
        height: parent.height
        onClicked: window.toggleMaximized()
    }

    Button {
        id: minimizeButton

        anchors.right: maximizeButton.left
        anchors.verticalCenter: parent.verticalCenter
        display: AbstractButton.IconOnly
        flat: true
        icon.height: 16
        icon.source: "qrc:/resources/assets/line-horizontal-1.svg"
        icon.width: 16
        radius: 0
        width: 48
        height: parent.height
        onClicked: window.showMinimized()
    }

    Button {
        id: styleButton

        anchors.right: minimizeButton.left
        anchors.verticalCenter: parent.verticalCenter
        display: AbstractButton.IconOnly
        flat: true
        icon.height: 16
        icon.source: Style.isDark ? "qrc:/resources/assets/weather-moon.svg" : "qrc:/resources/assets/weather-sunny.svg"
        icon.width: 16
        radius: 0
        width: 48
        height: parent.height
        onClicked: () => {
            Style.isDark = !Style.isDark;
            ui.isDark = Style.isDark;
        }
    }

    TapHandler {
        gesturePolicy: TapHandler.DragThreshold
        onTapped: {
            if (tapCount === 2)
                window.toggleMaximized();

        }
    }

    DragHandler {
        grabPermissions: TapHandler.DragThreshold
        onActiveChanged: {
            if (active)
                window.startSystemMove();

        }
    }

}
