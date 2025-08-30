#!/usr/bin/env bash

echo "Linux deployment script for WebSocket Reflector X"

export APP_NAME="WebSocketReflectorX"

if [ ${PWD##*/} != $APP_NAME ]; then
  echo "This script MUST be run from the $APP_NAME/ directory"
  exit 1
fi

echo '---- Running make install'
mkdir -p dist
APP_ROOT=dist
cp ./target/release/wsrx.exe "${APP_ROOT}/wsrx.exe"
cp ./target/release/wsrx-desktop.exe "${APP_ROOT}/wsrx-desktop.exe"

mv $APP_ROOT $APP_NAME

echo '---- Compressing package'
7z a $APP_NAME-portable-windows-msvc-x86_64.zip $APP_NAME

echo '---- Creating installer'
mv $APP_NAME windows/$APP_NAME
cp windows/$APP_NAME.ico windows/$APP_NAME/$APP_NAME.ico
makensis windows/setup.nsi
mv windows/*.exe $APP_NAME-installer-windows-msvc-x86_64.exe
