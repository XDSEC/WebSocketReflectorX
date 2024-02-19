import QtQuick
import QtQuick.Layouts
import QtQuick.Window
import Rx.Widgets

Item {
    id: root

    property string type
    property string remoteAddr
    property string tcpAddr
    property string wsAddr
    property int mlatency

    height: 70

    Rectangle {
        width: 850
        height: 70
        color: hoverHandler.hovered ? Style.palette.button : "transparent"
        Behavior on color {
            ColorAnimation {
                duration: Style.midAnimationDuration
            }
        }
        HoverHandler {
            id: hoverHandler
        }
        Rectangle {
            anchors.centerIn: parent
            anchors.topMargin: 5
            width: 830
            height: 50
            color: "transparent"
            
            Row {
                Rectangle {
                    width: 738
                    height: parent.height
                    color: "transparent"
                    Column {
                        spacing: 8
                        Row {
                            spacing: 30
                            Label {
                                text: remoteAddr + " -> " + tcpAddr
                            }
                            Label {
                                text: mlatency == -1 ? qsTr("Unreachable") : mlatency + " ms"
                                color: mlatency == -1 ? "red" : (latency < 300 ? "green": "yellow")
                            }
                        }
                        Label {
                            text: wsAddr
                            opacity: 0.6
                        }
                    }
                }
                

                Row {
                    Button {
                        id: copyButton
                        display: AbstractButton.IconOnly
                        borderWidth: 0
                        flat: true
                        icon.source: "qrc:/resources/assets/copy.svg"
                        icon.width: 20
                        icon.height: 20
                        icon.color: "green"
                        opacity: hoverHandler.hovered ? 1 : 0
                        onClicked: {
                            api.copyToClipboard(tcpAddr)
                        }
                        states: State {
                            name: "pressed"; when: copyButton.pressed
                            PropertyChanges { target: copyButton; scale: 1.3 }
                        }
                        transitions: Transition {
                            NumberAnimation { properties: "scale"; duration: 200; easing.type: Easing.InOutQuad }
                        }
                        Behavior on opacity {
                            NumberAnimation {
                                duration: Style.midAnimationDuration
                            }
                        }
                        ToolTip {
                            parent: copyButton
                            visible: parent.hovered
                            text: qsTr("Copy local address")
                        }
                    }
                    Button {
                        id: dismissButtion
                        display: AbstractButton.IconOnly
                        borderWidth: 0
                        flat: true
                        icon.source: "qrc:/resources/assets/dismiss.svg"
                        icon.width: 20
                        icon.height: 20
                        opacity: hoverHandler.hovered ? 1 : 0
                        onClicked: {
                            api.cancelClient(remoteAddr, wsAddr, tcpAddr, mlatency, type)
                        }
                        Behavior on opacity {
                            NumberAnimation {
                                duration: Style.midAnimationDuration
                            }
                        }
                        ToolTip {
                            parent: dismissButtion
                            visible: parent.hovered
                            text: qsTr("Delete connection")
                        }
                    }
                }
            }
        }
        Rectangle {
            anchors.left: parent.left
            anchors.top: parent.bottom
            width: parent.width
            height: 1
            color: Style.palette.button
        }
    }
}