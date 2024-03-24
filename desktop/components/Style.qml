import QtQuick
pragma Singleton

QtObject {
    id: rxStyle

    property ColorGroup darkPalette
    property bool isDark: true
    property ColorGroup lightPalette
    property ColorGroup palette: isDark ? darkPalette : lightPalette
    property int shortAnimationDuration: 120
    property int midAnimationDuration: 240
    property int longAnimationDuration: 480

    darkPalette: ColorGroup {
        property color primary: "#3399FF"
        property color info: "#3399FF"
        property color success: "#36D399"
        property color warning: "#FBBD23"
        property color error: "#F83030"
        property color debug: "#808080"

        alternateBase: "#202020"
        base: "transparent"
        brightText: "#ffffff"
        button: "#10ffffff"
        buttonText: "#f0f0f0"
        dark: "#30ffffff"
        highlight: "#403399FF"
        highlightedText: "#ffffff"
        light: "#10ffffff"
        link: primary
        linkVisited: Qt.darker(primary, 1.2)
        mid: "#28ffffff"
        midlight: "#18ffffff"
        placeholderText: "#40f0f0f0"
        shadow: "#000000"
        text: "#f0f0f0"
        toolTipBase: "#202020"
        toolTipText: "#f0f0f0"
        window: "#151515"
        windowText: "#f0f0f0"
    }

    lightPalette: ColorGroup {
        property color primary: "#0078D6"
        property color info: "#0078D6"
        property color success: "#36AA3A"
        property color warning: "#CA9f00"
        property color error: "#F83030"
        property color debug: "#808080"

        alternateBase: "#10000000"
        base: "transparent"
        brightText: "#000000"
        button: "#10000000"
        buttonText: "#101010"
        dark: "#40000000"
        highlight: "#403399FF"
        highlightedText: "#000000"
        light: "#00000000"
        link: primary
        linkVisited: Qt.darker(primary, 1.2)
        mid: "#30000000"
        midlight: "#20000000"
        placeholderText: "#40000000"
        shadow: "#000000"
        text: "#101010"
        toolTipBase: "#dee4e9"
        toolTipText: "#101010"
        window: "#eef4f9"
        windowText: "#101010"
    }

}
