#!/usr/bin/env bash

export APP_NAME="WebSocketReflectorX"
export APP_VERSION=0.2.6
export GIT_VERSION=$(git rev-parse --short HEAD)

echo "> $APP_NAME packager (macOS x86_64) [v$APP_VERSION]"

## CHECKS ######################################################################

if [ "$(id -u)" == "0" ]; then
  echo "This script MUST NOT be run as root" 1>&2
  exit 1
fi

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

## DEPLOY ######################################################################

CPU_ARCH=$(uname -m)

if [[ $use_contribs = true ]] ; then
  export LD_LIBRARY_PATH=$(pwd)/contribs/src/env/macOS_x86_64/usr/lib/
else
  export LD_LIBRARY_PATH=/usr/local/lib/
fi

echo '---- Running macdeployqt'
cp build/bin/wsrx build/bin/wsrx-desktop.app/Contents/MacOS/wsrx
cp -r build/bin/wsrx-desktop.app $APP_NAME.app
macdeployqt $APP_NAME.app -qmldir=./desktop/components -qmldir=./desktop/ui -hardened-runtime -timestamp -appstore-compliant -dmg

#echo '---- Installation directory content recap (after macdeployqt):'
#find bin/

## PACKAGE (zip) ###############################################################

if [[ $create_package = true ]] ; then
  echo '---- Compressing package'
  zip -r -y -X $APP_NAME-$APP_VERSION-macOS-$CPU_ARCH.zip $APP_NAME.app
  mv $APP_NAME.dmg $APP_NAME-$APP_VERSION-macOS-$CPU_ARCH.dmg
fi

## UPLOAD ######################################################################

if [[ $upload_package = true ]] ; then
  printf "---- Uploading to transfer.sh"
  curl --upload-file $APP_NAME*.zip https://transfer.sh/$APP_NAME.$APP_VERSION-git$GIT_VERSION-macOS-$CPU_ARCH.zip
  curl --upload-file $APP_NAME*.dmg https://transfer.sh/$APP_NAME.$APP_VERSION-git$GIT_VERSION-macOS-$CPU_ARCH.dmg
  printf "\n"
fi
