#!/bin/sh
# Records the opening cinematic headless, frame by frame from the game itself, and encodes it to
# an MP4 at 30 frames a second, pixels kept sharp, with its sound.
#
#   tools/record_cinematic.sh [seconds] [out.mp4]
#
# Run from the repository root after `cargo build` in rust/. Needs ffmpeg.
set -eu

SECONDS_LONG="${1:-21}"
OUT="${2:-$HOME/Desktop/Bane-videa/intro/cinematic.mp4}"
FPS=30
FRAMES=$(python3 -c "print(int($SECONDS_LONG * $FPS))")
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

mkdir -p "$(dirname "$OUT")"
(cd rust && BANE_START=intro ./target/debug/bane --shot "$WORK/f.png" --seconds "$SECONDS_LONG" \
    --shots "$FRAMES" --interval "$(python3 -c "print(1 / $FPS)")" --size 960x540)
# The frames are numbered without a fixed width; give them one so ffmpeg reads them in order.
for frame in "$WORK"/f_*.png; do
    number=${frame##*_}
    number=${number%.png}
    mv "$frame" "$WORK/$(printf 'g_%05d.png' "$((10#$number))")"
done
# With the cinematic's own track (tools/make_cinematic_audio.py) under it, where there is one.
AUDIO=assets/audio/cinematic.flac
if [ -f "$AUDIO" ]; then
    ffmpeg -y -loglevel error -framerate "$FPS" -start_number 1 -i "$WORK/g_%05d.png" -i "$AUDIO" \
        -vf "scale=960:540:flags=neighbor,format=yuv420p" -c:v libx264 -crf 20 -c:a aac -b:a 160k \
        -shortest -movflags +faststart "$OUT"
else
    ffmpeg -y -loglevel error -framerate "$FPS" -start_number 1 -i "$WORK/g_%05d.png" \
        -vf "scale=960:540:flags=neighbor,format=yuv420p" -c:v libx264 -crf 20 -movflags +faststart "$OUT"
fi
echo "$OUT"
