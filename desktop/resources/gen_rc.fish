echo '<RCC>' > resources.qrc
echo '  <qresource prefix="/resources">' >> resources.qrc
for i in (fd -e svg -e ttf)
  echo "    <file>$i</file>" >> resources.qrc
end
echo '  </qresource>' >> resources.qrc
echo '</RCC>' >> resources.qrc
