#!/bin/sh
# Builds dist/zspeak.app from a release binary.
#   packaging/macos/bundle.sh [path/to/zspeak]
# Signs ad hoc unless MACOS_SIGNING_IDENTITY names a Developer ID certificate in the keychain.
set -e
cd "$(dirname "$0")/../.."
BIN=${1:-target/release/zspeak}
APP=dist/zspeak.app
VERSION=$(grep -m1 '^version' client/Cargo.toml | cut -d'"' -f2)
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/zspeak"
cp packaging/icon/zspeak.icns "$APP/Contents/Resources/zspeak.icns"
sed "s/__VERSION__/$VERSION/g" packaging/macos/Info.plist > "$APP/Contents/Info.plist"
if [ -n "$MACOS_SIGNING_IDENTITY" ]; then
  codesign --force --options runtime --timestamp \
    --entitlements packaging/macos/entitlements.plist \
    --sign "$MACOS_SIGNING_IDENTITY" "$APP"
else
  codesign --force --options runtime --entitlements packaging/macos/entitlements.plist --sign - "$APP"
fi
codesign --verify --strict "$APP"
echo "built $APP ($VERSION)"
