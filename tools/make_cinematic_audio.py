"""The opening cinematic's sound: one track timed to its shots, built with ffmpeg from the
narrator's lines, the score and effects made with ElevenLabs (assets/audio/cinematic/, see
tools/make_narration.sh), a low synthesised drone and the game's own sounds.

    python3 tools/make_cinematic_audio.py [out.flac] [--stems DIR]

With --stems it also writes the score alone (no voice, no effects) to DIR, for listening to it
on its own. The timings follow SHOTS and the constants in rust/app/src/scenes/cinematic.rs.
"""

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
AUDIO = ROOT / "assets/audio"
MADE = AUDIO / "cinematic"

# The shots: temple, tear, crawl, crypt (two lines), as in cinematic.rs.
SHOTS = [11.0, 14.0, 10.0, 9.5, 5.5]
START = [sum(SHOTS[:i]) for i in range(len(SHOTS))]
LENGTH = sum(SHOTS)
TEMPLE, TEAR, CRAWL, CRYPT, RISE = START
LINE_FROM = 1.4
# The tear starts once its line is spoken; the angel bursts a little after.
TEAR_FROM = TEAR + 4.4
ANGEL_FALLS = TEAR_FROM + 1.9
# The crypt: the lid shakes, slides, lands; the hero rises.
LID_SHAKES = CRYPT + 2.8
LID_LANDS = CRYPT + 5.6 + 0.35
RISES = CRYPT + 8.4 - 0.25

args = [arg for i, arg in enumerate(sys.argv[1:], 1) if not arg.startswith("--") and sys.argv[i - 1] != "--stems"]
OUT = Path(args[0]) if args else AUDIO / "cinematic.flac"
STEMS = Path(sys.argv[sys.argv.index("--stems") + 1]) if "--stems" in sys.argv else None


class Mix:
    def __init__(self):
        self.inputs = []
        self.chains = []
        self.labels = []

    def _add(self, args, chain):
        index = len(self.labels)
        self.inputs.extend(args)
        label = f"s{index}"
        self.chains.append(f"[{index}:a]{chain}[{label}]")
        self.labels.append(label)

    def tone(self, expr, chain):
        self._add(["-f", "lavfi", "-t", str(LENGTH), "-i", expr], f"{chain},aformat=channel_layouts=stereo")

    def file(self, path, at, volume, chain="", fade_out=None):
        delay = int(at * 1000)
        extra = f"{chain}," if chain else ""
        tail = f",afade=t=out:st={fade_out[0]}:d={fade_out[1]}" if fade_out else ""
        self._add(["-i", str(path)],
                  f"{extra}aformat=channel_layouts=stereo,volume={volume}{tail},adelay={delay}|{delay}")

    def render(self, out):
        mix = "".join(f"[{label}]" for label in self.labels)
        graph = (";".join(self.chains) + f";{mix}amix=inputs={len(self.labels)}:normalize=0,"
                 f"alimiter=limit=0.9,apad,atrim=0:{LENGTH},afade=t=in:d=1.2,afade=t=out:st={LENGTH - 2}:d=2[out]")
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", *self.inputs, "-filter_complex", graph,
                        "-map", "[out]", "-c:a", "flac", "-ac", "2", str(out)],
                       check=True)


def window(start, rise, end, fall, peak):
    """A volume expression: silent, rising over `rise` from `start`, held, falling to `end`."""
    return (f"volume='{peak}*if(lt(t,{start}),0,if(lt(t,{start}+{rise}),(t-{start})/{rise},"
            f"if(lt(t,{end}-{fall}),1,if(lt(t,{end}),({end}-t)/{fall},0))))':eval=frame")


def score(mix):
    """The music under it all: the temple at peace, dread as the creatures come, the crypt."""
    def made(name, at, volume, fade_out=None):
        if (MADE / f"{name}.mp3").exists():
            mix.file(MADE / f"{name}.mp3", at, volume, fade_out=fade_out)

    # The temple bed runs from the start until the ground shakes, then fades.
    made("temple_bed", TEMPLE, 0.8, fade_out=(TEAR_FROM - TEMPLE, 2.5))
    # Under the tear and the crawl, a low drone of dread.
    for freq, amp in ((55, 0.2), (58.27, 0.1), (82.41, 0.06)):
        mix.tone(f"sine=frequency={freq}:sample_rate=44100",
                 f"tremolo=f=0.3:d=0.4,{window(TEAR_FROM - 0.5, 3.0, CRYPT, 1.0, amp)}")
    # The goosebumps: as the first creature climbs out, the dread swells.
    made("dread", CRAWL - 0.4, 1.0, fade_out=(CRYPT - CRAWL - 0.2, 1.2))
    # The crypt bed through to the end.
    made("crypt_bed", CRYPT + 0.4, 0.85)


def effects(mix):
    def made(name, at, volume):
        if (MADE / f"{name}.mp3").exists():
            mix.file(MADE / f"{name}.mp3", at, volume)

    made("rumble", TEAR_FROM - 0.1, 0.9)
    made("rumble", TEAR_FROM + 5.4, 0.55)
    made("statue", ANGEL_FALLS - 0.1, 0.9)
    # The rift tearing open: kept under the voice and the rumble.
    made("rift", TEAR_FROM + 1.0, 0.4)
    mix.file(AUDIO / "enemies/golem_slam.ogg", ANGEL_FALLS, 0.6)
    mix.file(AUDIO / "enemies/golem_rain.ogg", ANGEL_FALLS + 0.3, 0.4)
    made("crawl", CRAWL + 0.4, 0.8)
    made("crawl", CRAWL + 5.0, 0.5)
    for offset, name, volume in ((0.6, "enemies/spider_hatch.ogg", 0.5), (2.3, "enemies/spider_hatch.ogg", 0.4),
                                 (4.4, "enemies/spider_hatch.ogg", 0.35)):
        mix.file(AUDIO / name, CRAWL + offset, volume)
    made("lid", LID_SHAKES - 0.2, 1.0)
    mix.file(AUDIO / "enemies/golem_slam.ogg", LID_LANDS, 0.7)
    made("awaken", RISES - 0.5, 0.9)
    for freq, amp in ((1318.5, 0.12), (1760, 0.09), (2637, 0.04)):
        mix.tone(f"sine=frequency={freq}:sample_rate=44100",
                 f"volume='{amp}*if(lt(t,{RISES}),0,exp(-(t-{RISES})*1.4))':eval=frame")


def voice(mix):
    for number, at in enumerate((TEMPLE, TEAR, CRAWL, CRYPT, RISE), start=1):
        line = MADE / f"line_{number}.mp3"
        if line.exists():
            mix.file(line, at + LINE_FROM + 0.2, 1.7)


full = Mix()
score(full)
effects(full)
voice(full)
full.render(OUT)
print(OUT)
if STEMS:
    STEMS.mkdir(parents=True, exist_ok=True)
    alone = Mix()
    score(alone)
    alone.render(STEMS / "score_only.flac")
    print(STEMS / "score_only.flac")
