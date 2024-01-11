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
        acceptedButtons: Qt.NoButton // don't handle actual events
        // resize window mouse area
        anchors.fill: parent
        cursorShape: {
            if (root.state !== Window.Maximized) {
                const p = Qt.point(mouseX, mouseY);
                const b = 10;
                // Increase the corner size slightly
                if (p.x < b && p.y < b)
                    return Qt.SizeFDiagCursor;

                if (p.x >= width - b && p.y >= height - b)
                    return Qt.SizeFDiagCursor;

                if (p.x >= width - b && p.y < b)
                    return Qt.SizeBDiagCursor;

                if (p.x < b && p.y >= height - b)
                    return Qt.SizeBDiagCursor;

                if (p.x < b || p.x >= width - b)
                    return Qt.SizeHorCursor;

                if (p.y < b || p.y >= height - b)
                    return Qt.SizeVerCursor;

            }
        }
        hoverEnabled: true
    }

    DragHandler {
        id: resizeHandler

        grabPermissions: TapHandler.TakeOverForbidden
        target: null
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
