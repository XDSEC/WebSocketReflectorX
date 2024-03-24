import QtQuick
import Rx.Widgets

Item {
    // ----- Private Properties ----- //
    // ----- Private Functions ----- //

    // ----- Public Properties ----- //
    id: root

    property int radius: 25
    property bool running: false
    property color color: Style.palette.text
    property int _innerRadius: radius

    function _toRadian(degree) {
        return (degree * 3.14159) / 180;
    }

    function _getPosOnCircle(angleInDegree) {
        var centerX = root.width / 2, centerY = root.height / 2;
        var posX = 0, posY = 0;
        posX = centerX + root._innerRadius * Math.cos(_toRadian(angleInDegree));
        posY = centerY - root._innerRadius * Math.sin(_toRadian(angleInDegree));
        return Qt.point(posX, posY);
    }

    width: radius * 2
    height: radius * 2

    Repeater {
        id: repeater

        model: 4

        delegate: Component {
            Rectangle {
                // ----- Public Properties ----- //
                // ----- Private Functions ----- //

                id: rect

                // ----- Private Properties ----- //
                property int _currentAngle: _getStartAngle()

                function playAnimation() {
                    if (anim.running === false)
                        anim.start();
                    else
                        anim.resume();
                }

                function stopAnimation() {
                    anim.stop();
                }

                function _getStartAngle() {
                    var ang = 90;
                    return ang;
                }

                width: root.width / 10
                height: width
                radius: width / 2
                color: root.color
                transformOrigin: Item.Center
                x: root._getPosOnCircle(_currentAngle).x - width / 2
                y: root._getPosOnCircle(_currentAngle).y - width / 2
                antialiasing: true

                NumberAnimation {
                    id: anim

                    target: rect
                    property: "_currentAngle"
                    duration: 1000
                    from: rect._getStartAngle()
                    to: 360 + rect._getStartAngle()
                    easing.type: Easing.OutQuad
                }

            }

        }

    }

    Timer {
        // ----- Private Properties ----- //
        id: timer

        property int _circleIndex: 0

        interval: 1300
        triggeredOnStart: true
        repeat: true
        running: root.running
        onTriggered: {
            emitTimer.start();
        }
    }

    Timer {
        id: emitTimer

        property int _circleIndex: 0

        interval: 100
        triggeredOnStart: true
        repeat: true
        onTriggered: {
            var maxIndex = repeater.model;
            if (_circleIndex === maxIndex) {
                _circleIndex = 0;
                emitTimer.stop();
            } else {
                // console.log("_circleIndex: " + _circleIndex);
                repeater.itemAt(_circleIndex).playAnimation();
                _circleIndex++;
            }
        }
    }

}
