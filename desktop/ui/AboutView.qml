import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Rx.Widgets

Item {
    id: root

    Image {
        id: logoImage
        anchors.left: parent.left
        anchors.top: parent.top
        anchors.leftMargin: 32
        anchors.topMargin: 16
        source: "qrc:/resources/assets/logo.svg"
        sourceSize: Qt.size(48, 48)
    }

    Label {
        anchors.verticalCenter: logoImage.verticalCenter
        anchors.left: logoImage.right
        anchors.leftMargin: 16
        text: "WebSocket Reflector X"
        font.bold: true
        font.pixelSize: 24
    }
}
