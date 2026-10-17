#!/bin/sh
# The opening cinematic's narrator and sound effects, from ElevenLabs: each line spoken by a deep
# voice, then darkened (pitched down a little, with a crypt's echo), and a few effects made from
# text. The key is read from ~/.config/bane/elevenlabs.env (ELEVENLABS_API_KEY), never stored in
# the repository. Writes assets/audio/cinematic/; tools/make_cinematic_audio.py mixes them in.
#
#   tools/make_narration.sh [voice id]
set -eu

. "$HOME/.config/bane/elevenlabs.env"
VOICE="${1:-nPczCjzI2devNBz1zQrb}"
OUT=assets/audio/cinematic
mkdir -p "$OUT"

speak() {
    number=$1
    text=$2
    curl -sf -o "$OUT/raw_$number.mp3" -X POST \
        "https://api.elevenlabs.io/v1/text-to-speech/$VOICE?output_format=mp3_44100_128" \
        -H "xi-api-key: $ELEVENLABS_API_KEY" -H "Content-Type: application/json" \
        -d "{\"text\":\"$text\",\"model_id\":\"eleven_multilingual_v2\",\"voice_settings\":{\"stability\":0.55,\"similarity_boost\":0.8,\"style\":0.35,\"speed\":0.85}}"
    ffmpeg -y -loglevel error -i "$OUT/raw_$number.mp3" \
        -af "asetrate=44100*0.9,aresample=44100,atempo=1.05,lowpass=f=6500,aecho=0.8:0.6:120|260:0.25|0.15,volume=1.4" \
        "$OUT/line_$number.mp3"
    rm "$OUT/raw_$number.mp3"
}

effect() {
    name=$1
    seconds=$2
    text=$3
    curl -sf -o "$OUT/$name.mp3" -X POST "https://api.elevenlabs.io/v1/sound-generation" \
        -H "xi-api-key: $ELEVENLABS_API_KEY" -H "Content-Type: application/json" \
        -d "{\"text\":\"$text\",\"duration_seconds\":$seconds,\"prompt_influence\":0.6}"
}

speak 1 "Long ago... a temple stood on the hill."
speak 2 "Then something tore open beneath it."
speak 3 "They called it Bane. The wound."
speak 4 "One acolyte escaped... and woke the temple's last guardian."
speak 5 "Rise. Close the wound."

effect rumble 6 "deep earthquake rumble inside a stone temple, low and growing"
effect statue 3 "stone statue cracking and bursting apart, heavy chunks falling on stone floor"
effect rift 5 "a magical rift tearing open, crackling purple energy, otherworldly hum"
effect crawl 6 "many spiders and rats skittering and scratching out of a pit, dark and eerie"
effect lid 4 "heavy stone sarcophagus lid grinding and sliding open in a crypt"
effect awaken 3 "ethereal holy shimmer swelling, a soul returning to a body, deep breath"
effect cry 1.6 "a short muffled male death cry, then a wet bite and tearing, dark fantasy"
ls "$OUT"
