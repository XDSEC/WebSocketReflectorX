#!/usr/bin/env bash

export APP_NAME="WebSocketReflectorX"
export APP_VERSION=0.2.9
export GIT_VERSION=$(git rev-parse --short HEAD)

echo "> $APP_NAME packager (Linux x86_64) [v$APP_VERSION]"

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

## COPY FILES ##################################################################

mkdir -p dist
APP_ROOT=dist
install -D ./build/bin/wsrx "${APP_ROOT}/usr/bin/wsrx"
install -D ./build/bin/wsrx-desktop "${APP_ROOT}/usr/bin/wsrx-desktop"
install -Dm644 "./freedesktop/tech.woooo.wsrx.desktop" "$APP_ROOT"/usr/share/applications/tech.woooo.wsrx.desktop
install -Dm644 "./freedesktop/tech.woooo.wsrx.svg" "$APP_ROOT"/usr/share/icons/hicolor/scalable/apps/tech.woooo.wsrx.svg

## DEPLOY ######################################################################

if [[ $use_contribs = true ]] ; then
  export LD_LIBRARY_PATH=$(pwd)/contribs/src/env/linux_x86_64/usr/lib/:/usr/lib
else
  export LD_LIBRARY_PATH=/usr/lib/
fi

echo '---- Prepare linuxdeploy + plugins'

unset LD_LIBRARY_PATH; #unset QT_PLUGIN_PATH; #unset QTDIR;

USRDIR=/usr;
if [ -d bin/usr/local ]; then
  USRDIR=/usr/local
fi
if [ -z "$QTDIR" ]; then
  QTDIR=/usr/lib/qt
fi

if [ ! -x contribs/deploy/linuxdeploy-x86_64.AppImage ]; then
  wget -c -nv "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage" -P contribs/deploy/
  wget -c -nv "https://github.com/linuxdeploy/linuxdeploy-plugin-appimage/releases/download/continuous/linuxdeploy-plugin-appimage-x86_64.AppImage" -P contribs/deploy/
  wget -c -nv "https://github.com/linuxdeploy/linuxdeploy-plugin-qt/releases/download/continuous/linuxdeploy-plugin-qt-x86_64.AppImage" -P contribs/deploy/
fi
chmod a+x contribs/deploy/linuxdeploy-x86_64.AppImage
chmod a+x contribs/deploy/linuxdeploy-plugin-appimage-x86_64.AppImage
chmod a+x contribs/deploy/linuxdeploy-plugin-qt-x86_64.AppImage

# linuxdeploy settings
export EXTRA_QT_PLUGINS="svg;"
export EXTRA_PLATFORM_PLUGINS="libqxcb.so;libqwayland-egl.so;libqwayland-generic.so"
export QML_SOURCES_PATHS="desktop/ui:desktop/components"
## PACKAGE (AppImage) ##########################################################

if [[ $create_package = true ]] ; then
  echo '---- Running AppImage packager'
  ./contribs/deploy/linuxdeploy-x86_64.AppImage --appdir $APP_ROOT --plugin qt --output appimage
  mv $APP_NAME-x86_64.AppImage $APP_NAME-$APP_VERSION-linux64.AppImage
fi

## UPLOAD ######################################################################

if [[ $upload_package = true ]] ; then
  printf "---- Uploading to transfer.sh"
  curl --upload-file $APP_NAME*.AppImage https://transfer.sh/$APP_NAME-$APP_VERSION-git$GIT_VERSION-linux64.AppImage
  printf "\n"
fi
