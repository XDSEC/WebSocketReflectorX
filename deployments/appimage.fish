#!/usr/bin/env fish

echo "Linux deployment script for WebSocket Reflector X"

# test current directory is end with WebSocketReflectorX
if not string match -q -r 'WebSocketReflectorX$' (pwd)
    echo "Please run this script in the WebSocketReflectorX directory"
    exit 1
end

mkdir -p dist
set APP_ROOT dist
install -D ./target/release/wsrx $APP_ROOT/usr/bin/wsrx
install -D ./target/release/wsrx-desktop $APP_ROOT/usr/bin/wsrx-desktop
install -Dm644 ./freedesktop/tech.woooo.wsrx.desktop $APP_ROOT/usr/share/applications/tech.woooo.wsrx.desktop
install -Dm644 ./freedesktop/tech.woooo.wsrx.svg $APP_ROOT/usr/share/icons/hicolor/scalable/apps/tech.woooo.wsrx.svg

set USRDIR /usr
export NO_STRIP=true

if not test -f contribs/linuxdeploy-x86_64.AppImage
    wget -c -nv "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage" -P contribs/
    wget -c -nv "https://github.com/linuxdeploy/linuxdeploy-plugin-appimage/releases/download/continuous/linuxdeploy-plugin-appimage-x86_64.AppImage" -P contribs/
end

chmod a+x contribs/linuxdeploy-x86_64.AppImage
chmod a+x contribs/linuxdeploy-plugin-appimage-x86_64.AppImage

export EXTRA_PLATFORM_PLUGINS="libqxcb.so;libqwayland-egl.so;libqwayland-generic.so"

./contribs/linuxdeploy-x86_64.AppImage --appdir $APP_ROOT --output appimage
