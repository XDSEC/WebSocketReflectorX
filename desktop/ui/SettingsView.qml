import Qt.labs.platform
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Rx.Widgets

ScrollView {
    id: root

    clip: true
    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

    FileDialog {
        id: exportLogDialog

        fileMode: FileDialog.SaveFile
        folder: StandardPaths.writableLocation(StandardPaths.DocumentsLocation)
        nameFilters: [qsTr("Text files (*.txt)")]
        onAccepted: {
            daemon.exportLogs(exportLogDialog.file);
        }
    }

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
        anchors.right: parent.right
        anchors.rightMargin: 32
        anchors.top: logoImage.top
        anchors.topMargin: 8
        font.pixelSize: 24
        opacity: 0.8
    }

    Label {
        id: sloganText

        anchors.left: logoText.left
        anchors.top: logoText.bottom
        anchors.right: logoText.right
        anchors.topMargin: 8
        opacity: 0.6
        text: "Idealism is that you will probably never receive something back,\nbut nonetheless still decide to give."
    }

    ColumnLayout {
        spacing: 8
        anchors.top: sloganText.bottom
        anchors.topMargin: 48
        anchors.left: logoImage.left
        anchors.right: logoText.right

        RowLayout {
            Layout.fillWidth: true

            Label {
                text: qsTr("Software Language")
                Layout.fillWidth: true
            }

            ComboBox {
                id: languageComboBox

                textRole: "text"
                valueRole: "value"
                Layout.rightMargin: 8
                width: 240
                currentIndex: ["en_US", "zh_CN"].indexOf(ui.language)
                model: languageModel
                onCurrentValueChanged: {
                    ui.language = languageComboBox.currentValue;
                }

                ListModel {
                    id: languageModel

                    ListElement {
                        value: "en_US"
                        text: "English"
                    }

                    ListElement {
                        value: "zh_CN"
                        text: "简体中文"
                    }

                }

            }

        }

        Rectangle {
            height: 1
            color: Style.palette.midlight
            Layout.fillWidth: true
        }

        Switch {
            text: qsTr("Running in system tray when closed")
            checked: ui.runningInTray
            onCheckedChanged: {
                ui.runningInTray = checked;
            }
            Layout.fillWidth: true
            Layout.rightMargin: 8
        }

        Rectangle {
            height: 1
            color: Style.palette.midlight
            Layout.fillWidth: true
        }

        RowLayout {
            Layout.fillWidth: true

            Label {
                text: qsTr("Export network logs")
                Layout.fillWidth: true
            }

            Button {
                id: exportLogsButton

                Layout.rightMargin: 8
                icon.source: "qrc:/resources/assets/open.svg"
                icon.color: Style.palette.primary
                display: AbstractButton.TextBesideIcon
                text: qsTr("Export")
                onClicked: {
                    exportLogDialog.open();
                }
            }

        }

        Rectangle {
            height: 1
            color: Style.palette.midlight
            Layout.fillWidth: true
        }

        ColumnLayout {
            spacing: 8
            Layout.fillWidth: true

            Label {
                text: qsTr("System information for bug reporting and debugging")
                Layout.fillWidth: true
                Layout.topMargin: 8
            }

            Label {
                text: qsTr("Please include the following information when reporting bugs or asking for help.")
                Layout.fillWidth: true
                opacity: 0.6
            }

            ScrollView {
                id: debugInfoArea

                Layout.fillWidth: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

                TextArea {
                    id: innerDebugInfoArea

                    readOnly: true
                    text: daemon.systemInfo

                    Button {
                        id: copyButton

                        anchors.top: parent.top
                        anchors.topMargin: 8
                        anchors.right: parent.right
                        anchors.rightMargin: 8
                        display: AbstractButton.TextBesideIcon
                        icon.source: "qrc:/resources/assets/copy.svg"
                        icon.color: Style.palette.success
                        text: qsTr("Copy")
                        onClicked: {
                            innerDebugInfoArea.selectAll();
                            innerDebugInfoArea.copy();
                            innerDebugInfoArea.deselect();
                            icon.source = "qrc:/resources/assets/checkmark.svg";
                            text = qsTr("Copied!");
                            copyButtonTimer.start();
                        }

                        Timer {
                            id: copyButtonTimer

                            interval: 1000
                            repeat: false
                            onTriggered: {
                                copyButton.icon.source = "qrc:/resources/assets/copy.svg";
                                copyButton.text = qsTr("Copy");
                            }
                        }

                    }

                }

            }

        }

    }

    Label {
        id: copyrightText

        anchors.left: logoImage.left
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 16
        anchors.right: logoText.right
        text: qsTr("(c) 2022 - 2024 Ret 2 Shell, distributed with MIT license. Source code available at <a href=\"https://github.com/ret2shell/wsrx\">here</a>.")
        onLinkActivated: (link) => {
            return Qt.openUrlExternally(link);
        }
        opacity: 0.6
    }

}
