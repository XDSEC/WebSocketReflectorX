#!/usr/bin/env bash

export APP_NAME="WebSocketReflectorX"
export APP_VERSION=0.2.18
export GIT_VERSION=$(git rev-parse --short HEAD)

echo "> $APP_NAME packager (macOS x86_64) [v$APP_VERSION]"

if [ "$(id -u)" == "0" ]; then
  echo "This script MUST NOT be run as root" 1>&2
  exit 1
fi

if [ ${PWD##*/} != $APP_NAME ]; then
  echo "This script MUST be run from the $APP_NAME/ directory"
  exit 1
fi
CPU_ARCH=$(uname -m)

export LD_LIBRARY_PATH=$(pwd)/contribs/src/env/macOS_x86_64/usr/lib/

echo '---- Running macdeployqt'
strip build/bin/wsrx
cp build/bin/wsrx build/bin/wsrx-desktop.app/Contents/MacOS/wsrx
cp -r build/bin/wsrx-desktop.app $APP_NAME.app
macdeployqt $APP_NAME.app -qmldir=./desktop/components -qmldir=./desktop/ui -hardened-runtime -timestamp
find $APP_NAME.app/ -name "*.dSYM" | xargs rm -rf
sleep 3
hdiutil create $APP_NAME-tmp.dmg -ov -volname $APP_NAME -fs HFS+ -srcfolder ./$APP_NAME.app
hdiutil convert $APP_NAME-tmp.dmg -format UDZO -o $APP_NAME.dmg
mv $APP_NAME.dmg $APP_NAME-$APP_VERSION-macOS-$CPU_ARCH.dmg
