import QtQuick
import QtQuick.Controls
import Rx.Widgets

Button {
    id: root

    property bool isCurrent: false

    flat: true
    display: AbstractButton.TextBesideIcon
    alignment: Qt.AlignVCenter | Qt.AlignLeft
    font.bold: isCurrent

    Rectangle {
        id: indicator

        anchors.left: parent.left
        anchors.leftMargin: 2
        anchors.verticalCenter: parent.verticalCenter
        color: Style.palette.primary
        width: 4
        radius: 2
        height: root.isCurrent ? parent.height - 16 : 0

        Behavior on height {
            NumberAnimation {
                duration: Style.midAnimationDuration
                easing.type: Easing.InOutExpo
            }

        }

    }

}
