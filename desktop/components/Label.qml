import QtQuick
import QtQuick.Templates as T
import Rx.Widgets

T.Label {
    id: control

    property alias hovered: hoverHandler.hovered

    color: Style.palette.windowText
    linkColor: Style.palette.link
    elide: Text.ElideRight

    HoverHandler {
        id: hoverHandler
    }

}
