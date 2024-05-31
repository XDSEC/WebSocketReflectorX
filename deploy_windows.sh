#!/usr/bin/env bash

export APP_NAME="WebSocketReflectorX"
export APP_VERSION=0.2.16
export GIT_VERSION=$(git rev-parse --short HEAD)

echo "> $APP_NAME packager (Windows x86_64) [v$APP_VERSION]"

## CHECKS ######################################################################

if [ ${PWD##*/} != $APP_NAME ]; then
  echo "This script MUST be run from the $APP_NAME/ directory"
  exit 1
fi

echo '---- Running make install'
mkdir -p dist
APP_ROOT=dist
cp ./build/bin/wsrx.exe "${APP_ROOT}/wsrx.exe"
cp ./build/bin/wsrx-desktop.exe "${APP_ROOT}/wsrx-desktop.exe"

echo '---- Running windeployqt'
windeployqt $APP_ROOT/ --qmldir ./desktop/ui --qmldir ./desktop/components

mv $APP_ROOT $APP_NAME

echo '---- Compressing package'
7z a $APP_NAME-$APP_VERSION-win64.zip $APP_NAME

echo '---- Creating installer'
mv $APP_NAME windows/$APP_NAME
makensis windows/setup.nsi
mv windows/*.exe $APP_NAME-$APP_VERSION-win64.exe
