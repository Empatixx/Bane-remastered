#!/bin/sh
# Records the opening cinematic headless, frame by frame from the game itself, and encodes it to
# an MP4 at 30 frames a second, pixels kept sharp, with its sound.
#
#   tools/record_cinematic.sh [seconds] [out.mp4] [facebook.mp4]
#
# With a third path it also writes a 1080p copy for sharing, its size held under 24 MB (two-pass,
# its bitrate worked out from the length) so it fits Facebook's 25 MB limit.
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
SHARE="${3:-}"
if [ -n "$SHARE" ]; then
    # 24 MB over the length, less the sound's 160 kb/s, is what the picture may use.
    VIDEO_KBPS=$(python3 -c "print(int(24 * 8 * 1024 / $SECONDS_LONG - 160))")
    PASS="$WORK/pass"
    ffmpeg -y -loglevel error -framerate "$FPS" -start_number 1 -i "$WORK/g_%05d.png" \
        -vf "scale=1920:1080:flags=neighbor,format=yuv420p" -c:v libx264 -preset slow -b:v "${VIDEO_KBPS}k" \
        -pass 1 -passlogfile "$PASS" -an -f mp4 /dev/null
    ffmpeg -y -loglevel error -framerate "$FPS" -start_number 1 -i "$WORK/g_%05d.png" -i "$AUDIO" \
        -vf "scale=1920:1080:flags=neighbor,format=yuv420p" -c:v libx264 -preset slow -b:v "${VIDEO_KBPS}k" \
        -pass 2 -passlogfile "$PASS" -c:a aac -b:a 160k -shortest -movflags +faststart "$SHARE"
    echo "$SHARE"
fi
