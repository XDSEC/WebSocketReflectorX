import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Rx.Widgets

Item {
    id: root
    Rectangle {
        id: background
        anchors.left: parent.left
        anchors.leftMargin: 30
        anchors.rightMargin: 30
        anchors.top: parent.top

        Image {
            id: activeIcon
            anchors.top: parent.top
            anchors.left: parent.left
            sourceSize: Qt.size(16, 16)
            source: "qrc:/resources/assets/flash-on.svg"
        }
        Label {
            id: activeLabel
            anchors.left: activeIcon.right
            anchors.leftMargin: 15
            anchors.verticalCenter: activeIcon.verticalCenter
            text: qsTr("Active Connections")
            font.pixelSize: 18
        }

        Rectangle {
            id: noActive
            visible: api.noActiveClient
            anchors.left: parent.left
            anchors.top: activeLabel.bottom
            anchors.topMargin: 20
            width: 850
            height: 70
            color: Style.palette.window
            Text {
                anchors.verticalCenter: parent.verticalCenter
                anchors.horizontalCenter: parent.horizontalCenter
                color: Style.palette.windowText
                text: qsTr("No connections")
            }
        }

        ListView {
            id: activeClientsView
            anchors.left: parent.left
            anchors.top: activeLabel.bottom
            anchors.topMargin: 20
            clip: true
            ScrollBar.vertical: ScrollBar {}
            visible: !api.noActiveClient
            width: 850
            height: (70 * count + 20 * (count - 1)) <= 250 ? (70 * count + 20 * (count - 1)) : 250
            spacing: 20
            model: activeConnectionList

            add: Transition {
                NumberAnimation { properties: "x"; from: 100; duration: 300; easing.type: Easing.OutExpo; }
                NumberAnimation { properties: "opacity"; from: 0; to: 1; duration: 200; }
            }
            addDisplaced: Transition {
                NumberAnimation { properties: "y"; duration: 200; easing.type: Easing.OutExpo; }
            }
            removeDisplaced: Transition {
                PauseAnimation { duration: 300; }
                NumberAnimation { properties: "y"; duration: 200 }
            }
            remove: Transition {
                NumberAnimation { properties: "x"; to: 100; duration: 300; easing.type: Easing.OutExpo; }
                NumberAnimation { properties: "opacity"; from: 1; to: 0; duration: 300; easing.type: Easing.OutExpo; }
            }

            delegate: ConnectionDetail {
                type: "active"
                remoteAddr: remoteAddress
                tcpAddr: tcpAddress
                wsAddr: websocketAddress
                mlatency: latency
                height: 70
            }
        }
        
        Image {
            id: historyIcon
            anchors.top: api.noActiveClient ? noActive.bottom : activeClientsView.bottom
            anchors.left: parent.left
            anchors.topMargin: 20
            sourceSize: Qt.size(16, 16)
            source: "qrc:/resources/assets/plug-disconnected.svg"
        }

        Label {
            id: historyLabel
            anchors.left: historyIcon.right
            anchors.leftMargin: 15
            anchors.verticalCenter: historyIcon.verticalCenter
            text: qsTr("Disconnected Connections")
            font.pixelSize: 18
        }

        Rectangle {
            id: noHistory
            visible: api.noHistoryClient
            anchors.left: parent.left
            anchors.top: historyLabel.bottom
            anchors.topMargin: 20
            width: 850
            height: 70
            color: Style.palette.window
            Text {
                visible: api.noHistoryClient
                anchors.verticalCenter: parent.verticalCenter
                anchors.horizontalCenter: parent.horizontalCenter
                color: Style.palette.windowText
                text: qsTr("No connections")
            }
        }

        ListView {
            id: historyClientsView
            anchors.left: parent.left
            anchors.top: historyLabel.bottom
            anchors.topMargin: 20
            clip: true
            ScrollBar.vertical: ScrollBar {}
            visible: !api.noHistoryClient
            width: 850
            height: (70 * count + 20 * (count - 1)) <= 250 ? (70 * count + 20 * (count - 1)) : 250
            spacing: 20
            model: historyConnectionList

            add: Transition {
                NumberAnimation { properties: "x"; from: 100; duration: 300; easing.type: Easing.OutExpo; }
                NumberAnimation { properties: "opacity"; from: 0; to: 1; duration: 200; }
            }
            addDisplaced: Transition {
                NumberAnimation { properties: "y"; duration: 200; easing.type: Easing.OutExpo; }
            }
            removeDisplaced: Transition {
                PauseAnimation { duration: 300; }
                NumberAnimation { properties: "y"; duration: 200 }
            }
            remove: Transition {
                NumberAnimation { properties: "x"; to: 100; duration: 300; easing.type: Easing.OutExpo; }
                NumberAnimation { properties: "opacity"; from: 1; to: 0; duration: 300; easing.type: Easing.OutExpo; }
            }

            delegate: ConnectionDetail {
                type: "history"
                remoteAddr: remoteAddress
                tcpAddr: tcpAddress
                wsAddr: websocketAddress
                mlatency: latency
                height: 70
            }
        }
    }
}
