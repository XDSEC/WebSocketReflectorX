import QtQuick
import QtQuick.Layouts
import QtQuick.Window
import Rx.Widgets

Window {
    id: root

    function toggleMaximized() {
        if (root.visibility === Window.Maximized)
            root.showNormal();
        else
            root.showMaximized();
    }

    color: "transparent"
    flags: Qt.Window | Qt.FramelessWindowHint
    visible: true
    Component.onCompleted: {
        setX(Screen.width / 2 - width / 2);
        setY(Screen.height / 2 - height / 2);
    }

    MouseArea {
        enabled: root.state === Window.Normal && !ui.isMac
        acceptedButtons: Qt.NoButton // don't handle actual events
        hoverEnabled: true
        cursorShape: Qt.SizeFDiagCursor
        anchors.left: parent.left
        anchors.top: parent.top
        width: 10
        height: 10
    }

    MouseArea {
        enabled: root.state === Window.Normal && !ui.isMac
        acceptedButtons: Qt.NoButton // don't handle actual events
        hoverEnabled: true
        cursorShape: Qt.SizeFDiagCursor
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        width: 10
        height: 10
    }

    MouseArea {
        enabled: root.state === Window.Normal && !ui.isMac
        acceptedButtons: Qt.NoButton // don't handle actual events
        hoverEnabled: true
        cursorShape: Qt.SizeBDiagCursor
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        width: 10
        height: 10
    }

    MouseArea {
        enabled: root.state === Window.Normal && !ui.isMac
        acceptedButtons: Qt.NoButton // don't handle actual events
        hoverEnabled: true
        cursorShape: Qt.SizeBDiagCursor
        anchors.right: parent.right
        anchors.top: parent.top
        width: 10
        height: 10
    }

    MouseArea {
        enabled: root.state === Window.Normal && !ui.isMac
        acceptedButtons: Qt.NoButton // don't handle actual events
        hoverEnabled: true
        cursorShape: Qt.SizeHorCursor
        anchors.left: parent.left
        anchors.top: parent.top
        anchors.topMargin: 10
        width: 10
        height: parent.height - 20
    }

    MouseArea {
        enabled: root.state === Window.Normal && !ui.isMac
        acceptedButtons: Qt.NoButton // don't handle actual events
        hoverEnabled: true
        cursorShape: Qt.SizeHorCursor
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.topMargin: 10
        width: 10
        height: parent.height - 20
    }

    MouseArea {
        enabled: root.state === Window.Normal && !ui.isMac
        acceptedButtons: Qt.NoButton // don't handle actual events
        hoverEnabled: true
        cursorShape: Qt.SizeVerCursor
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.leftMargin: 10
        width: parent.width - 20
        height: 10
    }

    MouseArea {
        enabled: root.state === Window.Normal && !ui.isMac
        acceptedButtons: Qt.NoButton // don't handle actual events
        hoverEnabled: true
        cursorShape: Qt.SizeVerCursor
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.leftMargin: 10
        width: parent.width - 20
        height: 10
    }

    DragHandler {
        id: resizeHandler

        grabPermissions: TapHandler.TakeOverForbidden
        target: null
        enabled: !ui.isMac
        onActiveChanged: {
            if (active && root.state !== Window.Maximized) {
                const p = resizeHandler.centroid.position;
                const b = 10;
                let e = 0;
                if (p.x < 2 * b)
                    e |= Qt.LeftEdge;

                if (p.x >= width - 2 * b)
                    e |= Qt.RightEdge;

                if (p.y < 2 * b)
                    e |= Qt.TopEdge;

                if (p.y >= height - 2 * b)
                    e |= Qt.BottomEdge;

                if (e !== 0)
                    root.startSystemResize(e);

            }
        }
    }

}
