"""The opening cinematic's sound: one track timed to its shots, built with ffmpeg from the
narrator's lines, the score and effects made with ElevenLabs (assets/audio/cinematic/, see
tools/make_narration.sh), a low synthesised drone and the game's own sounds.

    python3 tools/make_cinematic_audio.py [out.flac] [--stems DIR]

With --stems it also writes the score alone (no voice, no effects) to DIR, for listening to it
on its own. The timings follow SHOTS and the constants in rust/app/src/scenes/cinematic.rs.
"""

import json
import subprocess
import tempfile
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
# How loud each layer is, in LUFS: the narrator well above the effects, the score under both.
VOICE = -14
EFFECTS = -26
SCORE = -29
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


WORK = Path(tempfile.mkdtemp(prefix="bane-cinematic-"))


def normalised(path, loudness):
    """A copy of `path` brought to `loudness` LUFS, measured first and then set exactly
    (two-pass), as plain 44.1 kHz audio."""
    out = WORK / f"{path.stem}_{-loudness}.wav"
    if out.exists():
        return out
    probe = subprocess.run(["ffmpeg", "-nostats", "-i", str(path), "-af",
                            f"loudnorm=I={loudness}:TP=-2:LRA=11:print_format=json", "-f", "null", "-"],
                           capture_output=True, text=True, check=True).stderr
    found = json.loads(probe[probe.rindex("{"):probe.rindex("}") + 1])
    measured = (f"measured_I={found['input_i']}:measured_TP={found['input_tp']}:measured_LRA={found['input_lra']}:"
                f"measured_thresh={found['input_thresh']}:offset={found['target_offset']}:linear=true")
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-i", str(path), "-af",
                    f"loudnorm=I={loudness}:TP=-2:LRA=11:{measured},aresample=44100", "-ac", "2", str(out)], check=True)
    return out


class Mix:
    """Two buses: the bed (score and effects) and the voice. The bed ducks under the voice
    whenever the narrator speaks, so the words always sit clearly on top."""

    def __init__(self):
        self.inputs = []
        self.chains = []
        self.buses = {"bed": [], "voice": []}

    def _add(self, args, chain, bus):
        index = len(self.inputs_count())
        self.inputs.extend(args)
        label = f"s{index}"
        self.chains.append(f"[{index}:a]{chain}[{label}]")
        self.buses[bus].append(label)

    def inputs_count(self):
        return [arg for arg in self.inputs if arg == "-i"]

    def tone(self, expr, chain):
        self._add(["-f", "lavfi", "-t", str(LENGTH), "-i", expr], f"{chain},aformat=channel_layouts=stereo", "bed")

    def file(self, path, at, volume, chain="", fade_out=None, bus="bed", loudness=None):
        """A file at `at` seconds. Unless told how, it eases in and dies away over its last
        stretch, so nothing starts or stops with a click."""
        delay = int(at * 1000)
        extra = f"{chain}," if chain else ""
        # Every clip brought to a set loudness first, so the voice, the effects and the score
        # each sit at their own level however loud the source was.
        if loudness is not None:
            path = normalised(path, loudness)
        if fade_out is None:
            length = float(subprocess.run(["ffprobe", "-v", "error", "-show_entries", "format=duration", "-of", "csv=p=0",
                                           str(path)], capture_output=True, text=True, check=True).stdout)
            tail_len = min(0.8, length * 0.3)
            fade_out = (max(0.0, length - tail_len), tail_len)
        tail = f",afade=t=in:d=0.04,afade=t=out:st={fade_out[0]}:d={fade_out[1]}"
        self._add(["-i", str(path)],
                  f"{extra}aformat=channel_layouts=stereo,volume={volume}{tail},adelay={delay}|{delay}", bus)

    def render(self, out):
        def bus(name):
            labels = self.buses[name]
            return "".join(f"[{label}]" for label in labels) + f"amix=inputs={len(labels)}:normalize=0,apad,atrim=0:{LENGTH}"

        parts = list(self.chains)
        if self.buses["voice"]:
            # The bed sits well under the narrator even where he is silent.
            parts.append(f"{bus('bed')}[bed]")
            # The narrator split: once to steer the duck, once to be heard.
            parts.append(f"{bus('voice')}[voice]")
            parts.append("[voice]asplit=2[key][said]")
            parts.append("[bed][key]sidechaincompress=threshold=0.015:ratio=10:attack=30:release=600:makeup=1[ducked]")
            parts.append("[ducked][said]amix=inputs=2:normalize=0[all]")
        else:
            parts.append(f"{bus('bed')}[all]")
        graph = ";".join(parts) + (f";[all]apad,atrim=0:{LENGTH},alimiter=limit=0.9,afade=t=in:d=1.2,"
                                   f"afade=t=out:st={LENGTH - 1.2}:d=1.2[out]")
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
            mix.file(MADE / f"{name}.mp3", at, volume, fade_out=fade_out, loudness=SCORE)

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
            mix.file(MADE / f"{name}.mp3", at, volume, loudness=EFFECTS)

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
    # The acolytes caught as the creatures pour out (the times the scene logs: "an acolyte is
    # caught", seconds into the crawl).
    for caught in (1.77, 1.9, 3.77, 4.88):
        made("cry", CRAWL + caught - 0.1, 0.8)
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
            mix.file(line, at + LINE_FROM + 0.2, 1.0, fade_out=(30.0, 0.1), bus="voice", loudness=VOICE)


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
