import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Rx.Widgets

Item {
    id: root

    Image {
        id: logoImage
        anchors.left: parent.left
        anchors.leftMargin: 32
        anchors.top: parent.top
        anchors.topMargin: 16

        source: "qrc:/resources/assets/logo.svg"
        sourceSize: Qt.size(96, 96)
        width: 96
        height: 96
    }

    Label {
        id: logoText
        text: "Web Socket Reflector X"
        anchors.left: logoImage.right
        anchors.leftMargin: 16
        anchors.top: logoImage.top
        anchors.topMargin: 4
        font.pixelSize: 24
        opacity: 0.8
    }

    Label {
        id: sloganText
        anchors.left: logoText.left
        anchors.top: logoText.bottom
        anchors.topMargin: 8
        opacity: 0.3
        text: "Idealism is that you will probably never receive something back,\nbut nonetheless still decide to give."
    }
}
