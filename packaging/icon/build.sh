#!/bin/sh
# Regenerates the icon files from zspeak.svg. Needs rsvg-convert, iconutil (macOS) and magick.
set -e
cd "$(dirname "$0")"
rm -rf zspeak.iconset && mkdir zspeak.iconset
for s in 16 32 128 256 512; do
  rsvg-convert -w $s -h $s zspeak.svg -o zspeak.iconset/icon_${s}x${s}.png
  rsvg-convert -w $((s*2)) -h $((s*2)) zspeak.svg -o zspeak.iconset/icon_${s}x${s}@2x.png
done
iconutil -c icns zspeak.iconset -o zspeak.icns
rsvg-convert -w 256 -h 256 zspeak.svg -o zspeak-256.png
rsvg-convert -w 512 -h 512 zspeak.svg -o zspeak-512.png
magick zspeak.iconset/icon_16x16.png zspeak.iconset/icon_32x32.png zspeak.iconset/icon_32x32@2x.png \
  zspeak.iconset/icon_128x128.png zspeak-256.png zspeak.ico
rm -rf zspeak.iconset
