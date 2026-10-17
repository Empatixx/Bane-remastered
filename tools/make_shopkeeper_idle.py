"""The shopkeeper's idle: he stands still with his hand where it is, the hem of his robe sways
from side to side, and once a loop his gold eyes blink.

Builds every frame from the rest pose, frame 0 of shopkeeper.tga (as dress_shopkeeper.py left
it), so running it again changes nothing. Rows above the hand never move; below it each row of
the robe slides sideways by whole pixels, more the lower it hangs. The blink paints the eyes over
with the dark inside of the hood for one frame.

    uv run -q --with pillow python tools/make_shopkeeper_idle.py [sheet] [--preview out.png]
"""

import math
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
FRAME = (80, 90)
DARK = (25, 15, 32, 255)
GLOW = (255, 214, 110, 255)

# One sway takes this many frames; the loop holds two of them so the blink comes less often.
SWAY_FRAMES = 12
FRAMES = SWAY_FRAMES * 2
BLINK = 17
# The robe starts to give just under the hand and swings this far at the hem.
STILL_ABOVE = 52
SWING = 2.0
PREVIEW_BG = (30, 24, 34, 255)


def offset(y, bottom, phase):
    if y <= STILL_ABOVE:
        return 0
    t = (y - STILL_ABOVE) / (bottom - STILL_ABOVE)
    return round(SWING * t ** 1.3 * math.sin(phase))


def sway(rest, phase):
    width, height = rest.size
    src = rest.load()
    bottom = max(y for y in range(height) for x in range(width) if src[x, y][3])
    out = Image.new("RGBA", rest.size, (0, 0, 0, 0))
    dst = out.load()
    for y in range(height):
        dx = offset(y, bottom, phase)
        for x in range(width):
            if 0 <= x - dx < width:
                dst[x, y] = src[x - dx, y]
    return out


def blink(frame):
    px = frame.load()
    width, height = frame.size
    for y in range(height):
        for x in range(width):
            if px[x, y] == GLOW:
                px[x, y] = DARK


def build(rest):
    frames = []
    for index in range(FRAMES):
        frame = sway(rest, 2 * math.pi * index / SWAY_FRAMES)
        if index == BLINK:
            blink(frame)
        frames.append(frame)
    return frames


def main():
    args = sys.argv[1:]
    preview = None
    if "--preview" in args:
        at = args.index("--preview")
        preview = Path(args[at + 1])
        del args[at:at + 2]
    path = Path(args[0]) if args else ROOT / "assets/textures/ingame/shopkeeper.tga"
    rest = Image.open(path).convert("RGBA").crop((0, 0, *FRAME))
    frames = build(rest)
    sheet = Image.new("RGBA", (FRAME[0] * FRAMES, FRAME[1]), (0, 0, 0, 0))
    for index, frame in enumerate(frames):
        sheet.paste(frame, (index * FRAME[0], 0))
    if preview:
        dark = Image.new("RGBA", sheet.size, PREVIEW_BG)
        dark.alpha_composite(sheet)
        dark.resize((sheet.width * 4, sheet.height * 4), Image.NEAREST).save(preview)
        gif = []
        for frame in frames:
            still = Image.new("RGBA", frame.size, PREVIEW_BG)
            still.alpha_composite(frame)
            gif.append(still.resize((FRAME[0] * 6, FRAME[1] * 6), Image.NEAREST).convert("RGB"))
        gif[0].save(preview.with_suffix(".gif"), save_all=True, append_images=gif[1:], duration=200, loop=0)
        return
    sheet.save(path)


if __name__ == "__main__":
    main()
