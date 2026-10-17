"""The opening cinematic's sound: one track timed to its shots, built with ffmpeg out of
synthesised pads and noise and the game's own sound effects, plus the narrator's lines when
they exist (assets/audio/cinematic/line_N.mp3, see tools/make_narration.sh).

    python3 tools/make_cinematic_audio.py [out.ogg]

The shot timings follow SHOTS in rust/app/src/scenes/cinematic.rs:
temple 0-11, tear 11-21, crawl 21-31, cut to black at 31, crypt 31-46 (the lid shakes at
33.8, slides at 34.8, lands at 36.95; the hero rises at 39.15).
"""

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
AUDIO = ROOT / "assets/audio"
LENGTH = 46.0
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else AUDIO / "cinematic.ogg"

# Where each narrator line starts, just after its subtitle begins typing.
LINES = [1.8, 12.4, 22.4, 32.4, 41.6]
CUT = AUDIO / "cinematic"

inputs = []
chains = []


def source(expr):
    inputs.extend(["-f", "lavfi", "-t", str(LENGTH), "-i", expr])
    return len(inputs) // 6 - 1 if False else None


def add_input(args):
    inputs.extend(args)


streams = []
index = 0


def lavfi(expr, chain):
    """A synthesised source run through `chain`, its volume shaped over time."""
    global index
    add_input(["-f", "lavfi", "-t", str(LENGTH), "-i", expr])
    label = f"s{index}"
    chains.append(f"[{index}:a]{chain}[{label}]")
    streams.append(label)
    index += 1


def effect(name, at, volume=1.0, chain=""):
    """One of the game's sounds, at `at` seconds."""
    global index
    add_input(["-i", str(AUDIO / name)])
    label = f"s{index}"
    delay = int(at * 1000)
    extra = f"{chain}," if chain else ""
    chains.append(f"[{index}:a]{extra}aformat=channel_layouts=stereo,volume={volume},adelay={delay}|{delay}[{label}]")
    streams.append(label)
    index += 1


def window(start, rise, end, fall, peak):
    """A volume expression: silent, rising over `rise` from `start`, held, falling to `end`."""
    return (f"volume='{peak}*if(lt(t,{start}),0,if(lt(t,{start}+{rise}),(t-{start})/{rise},"
            f"if(lt(t,{end}-{fall}),1,if(lt(t,{end}),({end}-t)/{fall},0))))':eval=frame")


# The temple at peace: a soft A-minor pad breathing slowly, and a little wind.
for freq, amp in ((110, 0.10), (164.81, 0.07), (220, 0.06), (261.63, 0.04)):
    lavfi(f"sine=frequency={freq}:sample_rate=44100",
          f"tremolo=f=0.18:d=0.5,{window(0, 2.5, 12.5, 2.0, amp)},aformat=channel_layouts=stereo")
lavfi("anoisesrc=color=pink:sample_rate=44100:amplitude=0.5",
      f"highpass=f=900,lowpass=f=3000,{window(0, 3, 31, 3, 0.05)},aformat=channel_layouts=stereo")

# The tear: a low rumble building from the first tremor, a hiss of chaos as the rift runs,
# and under it all a dark drone that stays till the cut.
lavfi("anoisesrc=color=brown:sample_rate=44100:amplitude=0.9",
      f"lowpass=f=110,{window(11.3, 2.5, 26.0, 4.0, 0.9)},aformat=channel_layouts=stereo")
lavfi("anoisesrc=color=pink:sample_rate=44100:amplitude=0.6",
      f"bandpass=f=1600:width_type=o:w=1.2,tremolo=f=11:d=0.6,{window(12.2, 1.0, 21.5, 3.5, 0.22)},aformat=channel_layouts=stereo")
for freq, amp in ((55, 0.22), (58.27, 0.12), (82.41, 0.08)):
    lavfi(f"sine=frequency={freq}:sample_rate=44100",
          f"tremolo=f=0.3:d=0.4,{window(11.5, 3.0, 31.0, 1.0, amp)},aformat=channel_layouts=stereo")

# The made effects (tools/make_narration.sh), where they exist.
def made(name, at, volume):
    if (CUT / f"{name}.mp3").exists():
        effect(f"cinematic/{name}.mp3", at, volume)


made("rumble", 11.2, 1.1)
made("rumble", 16.6, 0.7)
made("statue", 12.8, 1.0)
made("rift", 12.2, 0.9)
made("rift", 16.8, 0.5)
made("crawl", 21.4, 0.9)
made("crawl", 26.2, 0.6)
made("lid", 33.6, 1.0)
made("awaken", 38.6, 0.9)
effect("enemies/golem_slam.ogg", 36.95, 0.8)

# The angel shudders and bursts; its pieces land.
effect("enemies/golem_rage.ogg", 12.0, 0.5, "asetrate=44100*0.7,aresample=44100")
effect("enemies/golem_slam.ogg", 12.9, 1.0)
effect("enemies/king_smash.ogg", 12.95, 0.8, "asetrate=44100*0.8,aresample=44100")
effect("enemies/golem_rain.ogg", 13.2, 0.6)
effect("explosion.ogg", 12.9, 0.45, "asetrate=44100*0.6,aresample=44100")
# The rift tearing open.
effect("enemies/argus_charge.ogg", 12.3, 0.6, "asetrate=44100*0.55,aresample=44100")
effect("enemies/argus_rage.ogg", 15.0, 0.4, "asetrate=44100*0.6,aresample=44100")

# What crawls out.
for at, name, volume in ((21.6, "enemies/spider_hatch.ogg", 0.8), (22.6, "enemies/mimic_gurgle.ogg", 0.5),
                         (23.3, "enemies/spider_hatch.ogg", 0.7), (24.4, "enemies/web_spit.ogg", 0.5),
                         (25.4, "enemies/spider_hatch.ogg", 0.6), (26.6, "enemies/slime_burst.ogg", 0.4),
                         (27.4, "enemies/hound_growl.ogg", 0.35)):
    effect(name, at, volume)

# The crypt: a warmer pad, a swell as the light gathers, a bright chime at the rising.
for freq, amp in ((220, 0.07), (277.18, 0.05), (329.63, 0.05), (440, 0.025)):
    lavfi(f"sine=frequency={freq}:sample_rate=44100",
          f"tremolo=f=0.25:d=0.4,{window(31.6, 3.0, LENGTH, 3.0, amp)},aformat=channel_layouts=stereo")
lavfi("anoisesrc=color=pink:sample_rate=44100:amplitude=0.5",
      f"bandpass=f=2400:width_type=o:w=1,{window(33.4, 5.6, 39.6, 0.4, 0.12)},aformat=channel_layouts=stereo")
for freq, amp in ((1318.5, 0.16), (1760, 0.12), (2637, 0.06)):
    lavfi(f"sine=frequency={freq}:sample_rate=44100",
          f"volume='{amp}*if(lt(t,39.15),0,exp(-(t-39.15)*1.4))':eval=frame,aformat=channel_layouts=stereo")
effect("enemies/kin_heal.ogg", 39.1, 0.7)
effect("heal.ogg", 39.2, 0.5)

# The narrator, where there is one.
for number, at in enumerate(LINES, start=1):
    line = AUDIO / "cinematic" / f"line_{number}.mp3"
    if line.exists():
        add_input(["-i", str(line)])
        label = f"s{index}"
        delay = int(at * 1000)
        chains.append(f"[{index}:a]aformat=channel_layouts=stereo,volume=1.6,adelay={delay}|{delay}[{label}]")
        streams.append(label)
        index += 1

mix = "".join(f"[{label}]" for label in streams)
graph = ";".join(chains) + f";{mix}amix=inputs={len(streams)}:normalize=0,alimiter=limit=0.9,atrim=0:{LENGTH},afade=t=out:st={LENGTH - 2}:d=2[out]"
command = ["ffmpeg", "-y", "-loglevel", "error", "-nostats", *inputs, "-filter_complex", graph, "-map", "[out]",
           "-c:a", "vorbis", "-strict", "-2", "-ac", "2", "-b:a", "160k", str(OUT)]
subprocess.run(command, check=True)
print(OUT)
