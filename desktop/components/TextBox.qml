import QtQuick
import QtQuick.Controls
import Rx.Widgets

Rectangle {
    id: root

    property color focusBorderColor: Style.palette.mid
    property color focusColor: Style.palette.window
    property int fontSize: 16
    property color hoverBorderColor: Style.palette.dark
    property color hoverColor: Style.palette.midlight
    property alias inputArea: inputTextBox
    property alias inputText: inputTextBox.text
    property color normalBorderColor: "transparent"
    property color normalColor: Style.palette.button
    property string placeholder: ""

    signal enterPressed()
    signal escPressed()
    signal inputActive()
    signal inputEdited(string input)
    signal inputFinished(string input)
    signal inputInactive()
    signal inputRejected()
    signal tabPressed()

    function setInputFocus() {
        inputTextBox.forceActiveFocus();
    }

    border.width: 2
    radius: 6
    clip: true
    implicitHeight: 40
    state: "Normal"
    states: [
        State {
            name: "Hovering"

            PropertyChanges {
                border.color: hoverBorderColor
                color: hoverColor
                target: root
            }

        },
        State {
            name: "Normal"

            PropertyChanges {
                border.color: normalBorderColor
                color: normalColor
                target: root
            }

        },
        State {
            name: "Focus"

            PropertyChanges {
                border.color: focusBorderColor
                color: focusColor
                target: root
            }

        }
    ]

    TextInput {
        id: inputTextBox

        activeFocusOnPress: true
        anchors.left: parent.left
        anchors.leftMargin: 9
        anchors.right: parent.right
        anchors.rightMargin: 9
        anchors.verticalCenter: parent.verticalCenter
        // implicitWidth: root.width
        // canPaste: true
        clip: true
        font.pixelSize: fontSize
        height: parent.height
        color: Style.palette.text
        selectByMouse: true
        selectedTextColor: Style.palette.text
        selectionColor: Color.transparent(Style.palette.primary, 0.4)
        text: inputText
        verticalAlignment: Text.AlignVCenter
        wrapMode: TextEdit.NoWrap
        Keys.onPressed: function(event) {
            if (event.key === Qt.Key_Enter) {
                root.enterPressed();
                root.inputFinished(inputTextBox.text);
            } else if (event.key === Qt.Key_Esc) {
                root.escPressed();
            } else if (event.key === Qt.Key_Tab) {
                root.tabPressed();
                root.inputText = root.placeholder;
            }
        }
        onAccepted: {
            root.inputFinished(inputTextBox.text);
        }
        onActiveFocusChanged: {
            if (activeFocus) {
                root.state = "Focus";
                root.inputActive();
            } else {
                root.state = "Normal";
                root.inputInactive();
            }
        }
        onTextEdited: {
            root.inputEdited(inputTextBox.text);
        }

        MouseArea {
            acceptedButtons: Qt.RightButton
            anchors.fill: parent
            cursorShape: Qt.IBeamCursor
            hoverEnabled: true
            onClicked: {
                contentMenu.popup();
            }

            Menu {
                id: contentMenu

                MenuItem {
                    icon.source: "qrc:/resources/assets/add-square-multiple.svg"
                    text: qsTr("Select All")
                    onTriggered: inputTextBox.selectAll()
                }

                MenuItem {
                    icon.source: "qrc:/resources/assets/cut.svg"
                    text: qsTr("Cut")
                    onTriggered: inputTextBox.cut()
                }

                MenuItem {
                    icon.source: "qrc:/resources/assets/copy.svg"
                    text: qsTr("Copy")
                    onTriggered: inputTextBox.copy()
                }

                MenuItem {
                    icon.source: "qrc:/resources/assets/clipboard-paste.svg"
                    text: qsTr("Paste")
                    onTriggered: inputTextBox.paste()
                }

            }

        }

        cursorDelegate: Rectangle {
            id: cursorDelegate

            color: Color.transparent(Style.palette.buttonText, 0.6)
            width: 2

            Connections {
                function onActiveFocusChanged() {
                    if (target.activeFocus)
                        cursorAnimation.start();
                    else
                        cursorAnimation.stop();
                }

                function onCursorPositionChanged() {
                    if (target.activeFocus)
                        cursorAnimation.restart();

                }

                target: inputTextBox
            }

            SequentialAnimation {
                id: cursorAnimation

                loops: SequentialAnimation.Infinite
                running: false
                onStarted: {
                    cursorDelegate.visible = true;
                }
                onStopped: {
                    cursorDelegate.visible = false;
                }

                NumberAnimation {
                    duration: Style.midAnimationDuration
                    easing.type: Easing.OutCurve
                    from: 1
                    property: "opacity"
                    target: cursorDelegate
                    to: 0
                }

                PauseAnimation {
                    duration: Style.midAnimationDuration
                }

                NumberAnimation {
                    duration: Style.midAnimationDuration
                    easing.type: Easing.InCurve
                    from: 0
                    property: "opacity"
                    target: cursorDelegate
                    to: 1
                }

                PauseAnimation {
                    duration: Style.midAnimationDuration
                }

            }

        }

    }

    Text {
        id: placeHolderText

        anchors.fill: parent
        anchors.leftMargin: 9
        anchors.rightMargin: 3 + root.height
        color: "#808080"
        font.pixelSize: fontSize
        horizontalAlignment: Text.AlignLeft
        text: placeholder
        verticalAlignment: Text.AlignVCenter
        visible: inputTextBox.text === "" ? true : false
    }

    MouseArea {
        id: hoverArea

        anchors.fill: parent
        cursorShape: Qt.IBeamCursor
        hoverEnabled: parent.enabled
        propagateComposedEvents: true
        onClicked: function(mouse) {
            mouse.accepted = false;
        }
        onEntered: {
            if (!inputTextBox.activeFocus)
                root.state = "Hovering";

        }
        onExited: {
            if (!inputTextBox.activeFocus)
                root.state = "Normal";

        }
        onPressed: function(mouse) {
            mouse.accepted = false;
        }
    }

}
