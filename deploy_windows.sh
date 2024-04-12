#!/usr/bin/env bash

export APP_NAME="WebSocketReflectorX"
export APP_VERSION=0.2.3
export GIT_VERSION=$(git rev-parse --short HEAD)

echo "> $APP_NAME packager (Windows x86_64) [v$APP_VERSION]"

## CHECKS ######################################################################

if [ ${PWD##*/} != $APP_NAME ]; then
  echo "This script MUST be run from the $APP_NAME/ directory"
  exit 1
fi

## SETTINGS ####################################################################

use_contribs=false
make_install=false
create_package=false
upload_package=false

while [[ $# -gt 0 ]]
do
case $1 in
  -c|--contribs)
  use_contribs=true
  ;;
  -i|--install)
  make_install=true
  ;;
  -p|--package)
  create_package=true
  ;;
  -u|--upload)
  upload_package=true
  ;;
  *)
  echo "> Unknown argument '$1'"
  ;;
esac
shift # skip argument or value
done

## APP INSTALL #################################################################

echo '---- Running make install'
mkdir -p dist
APP_ROOT=dist
cp ./build/bin/wsrx.exe "${APP_ROOT}/wsrx.exe"
cp ./build/bin/wsrx-desktop.exe "${APP_ROOT}/wsrx-desktop.exe"
#echo '---- Installation directory content recap (after make install):'
#find bin/

## DEPLOY ######################################################################

echo '---- Running windeployqt'
windeployqt $APP_ROOT/ --qmldir ./desktop/ui --qmldir ./desktop/components

#echo '---- Installation directory content recap (after windeployqt):'
#find bin/

mv $APP_ROOT $APP_NAME

## PACKAGE (zip) ###############################################################

if [[ $create_package = true ]] ; then
  echo '---- Compressing package'
  7z a $APP_NAME-$APP_VERSION-win64.zip $APP_NAME
fi

## PACKAGE (NSIS) ##############################################################

if [[ $create_package = true ]] ; then
  echo '---- Creating installer'
  mv $APP_NAME windows/$APP_NAME
  makensis windows/setup.nsi
  mv windows/*.exe $APP_NAME-$APP_VERSION-win64.exe
fi

## UPLOAD ######################################################################

if [[ $upload_package = true ]] ; then
  printf "---- Uploading to transfer.sh"
  curl --upload-file $APP_NAME*.zip https://transfer.sh/$APP_NAME-$APP_VERSION-git$GIT_VERSION-win64.zip
  printf "\n"
  curl --upload-file $APP_NAME*.exe https://transfer.sh/$APP_NAME-$APP_VERSION-git$GIT_VERSION-win64.exe
  printf "\n"
fi
