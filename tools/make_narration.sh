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

# `ONLY="3 4 7 8 steps"` makes just those lines (by number) and effects (by name), to save
# credits when only some have changed.
wanted() {
    [ -z "${ONLY:-}" ] && return 0
    case " $ONLY " in *" $1 "*) return 0 ;; esac
    return 1
}
line() { wanted "$1" && speak "$1" "$2"; return 0; }
sound() { wanted "$1" && effect "$1" "$2" "$3"; return 0; }

line 1 "For four hundred years... the people of Velmor came to pray at the temple on the hill."
line 2 "Their prayers kept something asleep beneath it."
line 3 "One night... the brothers fell asleep at their prayers... one by one."
line 4 "Only Elian, the youngest, was still awake... Then he slept too."
line 5 "And something tore open beneath the temple."
line 6 "They called it Bane. The wound."
line 7 "Only Elian and Brother Aldric reached the crypt alive."
line 8 "Elian never told anyone... that he had fallen asleep."

sound rumble 6 "deep earthquake rumble inside a stone temple, low and growing"
sound statue 3 "stone statue cracking and bursting apart, heavy chunks falling on stone floor"
sound rift 5 "a magical rift tearing open, crackling purple energy, otherworldly hum"
sound crawl 6 "many spiders and rats skittering and scratching out of a pit, dark and eerie"
sound steps 4 "hurried footsteps of two people running into an echoing stone crypt, one limping, ragged breathing"
sound day_bed 22 "dark fantasy game soundtrack, a sacred temple by day, a calm choir and warm strings, peaceful and reverent, slow"
sound night_bed 10 "dark fantasy game soundtrack, night falls over a temple, the choir fades to silence, a low uneasy drone, wind"
sound cry 1.6 "a short muffled male death cry, then a wet bite and tearing, dark fantasy"
ls "$OUT"
