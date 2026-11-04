"""Writes the chibi hero's textures from the hand-drawn parts in tools/art:

- assets/textures/ingame/player.tga: 6x6 frames of 20x26 (rows in tools/art/chibi_elian.py);
- assets/textures/ingame/player_dodge.tga: the dodge roll, 6 frames of 24x32 (tools/art/chibi_roll.py):
  crouch, leap, a somersault in the air, landing;
- assets/textures/ingame/held/hand.tga: the floating hand that holds the gun;
- assets/textures/ingame/held/<gun>.tga: each gun as held, pointing right (tools/art/hand_guns.py).

The grip and muzzle of each held gun are written into rust/app/src/held.rs by hand from
hand_guns.py; this prints them to compare.

    uv run -q --with pillow python tools/make_chibi_player.py
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools/art"))

import chibi_elian  # noqa: E402
import chibi_roll  # noqa: E402
import hand_guns  # noqa: E402

OUT = ROOT / "assets/textures/ingame"
W, H = chibi_elian.W, chibi_elian.H

HAND = [
    "aaaa",
    "afia",
    "aiia",
    "aaaa",
]

def image(rows, palette):
    out = Image.new("RGBA", (len(rows[0]), len(rows)), (0, 0, 0, 0))
    px = out.load()
    for y, line in enumerate(rows):
        for x, key in enumerate(line):
            if key != ".":
                colour = palette[key].lstrip("#")
                px[x, y] = (int(colour[0:2], 16), int(colour[2:4], 16), int(colour[4:6], 16), 255)
    return out


def player_sheet():
    rows = chibi_elian.ROWS
    sheet = Image.new("RGBA", (W * 6, H * len(rows)), (0, 0, 0, 0))
    for r, frames in enumerate(rows):
        for c, frame in enumerate(frames):
            sheet.alpha_composite(image(frame, chibi_elian.PAL), (c * W, r * H))
    return sheet


def dodge_sheet():
    frames = chibi_roll.ROLL
    sheet = Image.new("RGBA", (chibi_roll.W * len(frames), chibi_roll.H), (0, 0, 0, 0))
    for c, frame in enumerate(frames):
        sheet.alpha_composite(image(frame, chibi_elian.PAL), (c * chibi_roll.W, 0))
    return sheet


def main():
    player_sheet().save(OUT / "player.tga")
    dodge_sheet().save(OUT / "player_dodge.tga")
    held = OUT / "held"
    held.mkdir(exist_ok=True)
    palette = dict(chibi_elian.PAL)
    image(HAND, palette).save(held / "hand.tga")
    for name, gun in hand_guns.GUNS.items():
        image(gun["rows"], hand_guns.PAL).save(held / f"{name}.tga")
        print(f"{name:13} size {len(gun['rows'][0])}x{len(gun['rows'])} grip {gun['grip']} muzzle {gun['muzzle']}")


if __name__ == "__main__":
    main()
