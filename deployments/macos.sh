#!/bin/bash

APP="target/release/bundle/osx/WebSocketReflectorX.app"
APP_NAME="WebSocketReflectorX"
BIN="$APP/Contents/MacOS/wsrx-desktop"
ZIP="./WebSocketReflectorX.app.zip"

cargo bundle --release
# codesign --timestamp --verify -vvv --deep --options=runtime --sign $IDENT $APP
zip -r $ZIP $APP
# xcrun notarytool submit --apple-id $USERNAME --team-id $IDENT --password $PASSWORD --wait $ZIP
# xcrun stapler staple $APP
hdiutil create $APP_NAME-tmp.dmg -ov -volname $APP_NAME -fs HFS+ -srcfolder $APP
hdiutil convert $APP_NAME-tmp.dmg -format UDZO -o $APP_NAME.dmg
