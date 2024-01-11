echo '<RCC>' > ui.qrc
echo '  <qresource prefix="/ui">' >> ui.qrc
for i in (fd -e qml)
  echo "    <file>$i</file>" >> ui.qrc
end
echo '  </qresource>' >> ui.qrc
echo '</RCC>' >> ui.qrc
