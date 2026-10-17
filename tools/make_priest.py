"""The temple's priests for the opening cinematic, drawn at low resolution: 10x19 pixel frames,
shown four times over (40x76, the hero's height) so their pixels are as chunky as the
dungeon's props. A hooded white robe with gold trim, the face in the hood's shadow.

Rows: 0 standing (four frames, a slow breath), 1 running (four frames, side view, facing
right), 2 praying (two frames, hands raised, the second with a glint of light).

    uv run -q --with pillow python tools/make_priest.py [preview.png]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
W, H = 10, 19

PALETTE = {
    ".": None,
    "k": (30, 22, 30),  # outline
    "w": (226, 218, 202),  # robe
    "s": (170, 160, 148),  # robe shade
    "d": (118, 108, 104),  # robe deep shade
    "g": (222, 182, 82),  # gold trim
    "G": (150, 112, 46),  # gold shade
    "h": (58, 46, 54),  # inside the hood
    "f": (214, 160, 118),  # face in shadow
    "F": (238, 195, 154),  # hands
    "y": (255, 240, 170),  # glint
}

STAND = [
    "...kkkk...",
    "..kwwwwk..",
    ".kwwhhwwk.",
    ".kwhffhwk.",
    ".kwhffhsk.",
    ".kswhhwsk.",
    "..kgggggk.",
    ".kwwwgwwsk",
    ".kwwwgwwsk",
    "kFwwwgwwsk",
    "kkwwwgwwsk",
    ".kwwwgwwsk",
    ".kwwwgwwsk",
    ".kswwgwssk",
    ".kswwgwssk",
    "kswwwgwsdk",
    "kswwwgwsdk",
    "kGgggggggk",
    "kkkkkkkkkk",
]

RUN = [
    "...kkkk...",
    "..kwwwwk..",
    "..kwwhhwk.",
    "..kwwfhwk.",
    "..kswffwk.",
    "..kswhwwk.",
    "..kgggggk.",
    ".kswwwwwk.",
    "kswwwwwFk.",
    "kswwwwwkk.",
    ".kswwwwwk.",
    ".kswwwwwk.",
    "..kswwwwk.",
    "..kswwwwk.",
    ".kswwwwwwk",
    "kswwwwwwsk",
    "kGggggggGk",
    "kk.kk.kk.k",
    "..........",
]

PRAY = [
    "...kkkk...",
    "..kwwwwk..",
    "kFkwhhwkFk",
    "kFwhffhwFk",
    ".kwhffhwk.",
    ".kswhhwsk.",
    "..kgggggk.",
    ".kwwwgwwsk",
    ".kwwwgwwsk",
    ".kwwwgwwsk",
    ".kwwwgwwsk",
    ".kwwwgwwsk",
    ".kwwwgwwsk",
    ".kswwgwssk",
    ".kswwgwssk",
    "kswwwgwsdk",
    "kswwwgwsdk",
    "kGgggggggk",
    "kkkkkkkkkk",
]


def draw(rows, lift=0, sway=0, legs=None, glint=False):
    image = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    px = image.load()
    for y, line in enumerate(rows):
        line = line.ljust(W, ".")[:W]
        # The upper body rises a pixel on a breath; the hem sways.
        dy = -lift if y < 7 else 0
        dx = sway if y >= 14 else 0
        for x, key in enumerate(line):
            colour = PALETTE[key]
            tx, ty = x + dx, y + dy
            if colour and 0 <= tx < W and 0 <= ty < H:
                px[tx, ty] = (*colour, 255)
    if legs is not None:
        # Running: the feet under the hem, alternating.
        for x in range(W):
            px[x, H - 2] = (0, 0, 0, 0)
            px[x, H - 1] = (0, 0, 0, 0)
        for x in legs:
            px[x, H - 2] = (*PALETTE["k"], 255)
            px[x, H - 1] = (*PALETTE["k"], 255)
    if glint:
        for x, y in ((0, 1), (9, 1), (1, 0), (8, 0)):
            px[x, y] = (*PALETTE["y"], 255)
    return image


def sheet():
    out = Image.new("RGBA", (W * 4, H * 3), (0, 0, 0, 0))
    for i, lift in enumerate((0, 0, 1, 1)):
        out.alpha_composite(draw(STAND, lift=lift, sway=1 if i == 2 else 0), (i * W, 0))
    for i, legs in enumerate(((2, 3, 6), (3, 4, 7), (4, 5, 7), (2, 6, 7))):
        out.alpha_composite(draw(RUN, lift=i % 2, legs=legs), (i * W, H))
    out.alpha_composite(draw(PRAY), (0, H * 2))
    out.alpha_composite(draw(PRAY, lift=1, glint=True), (W, H * 2))
    return out


def main():
    image = sheet()
    if len(sys.argv) > 1:
        hero = Image.open(ROOT / "assets/textures/ingame/player.tga").convert("RGBA").crop((0, 0, 16, 38))
        big = image.resize((W * 4 * 4, H * 3 * 4), Image.NEAREST)
        preview = Image.new("RGBA", (big.width + 16 * 2 + 16, big.height), (30, 28, 36, 255))
        preview.alpha_composite(big, (0, 0))
        preview.alpha_composite(hero.resize((32, 76), Image.NEAREST), (big.width + 8, 0))
        preview.resize((preview.width * 2, preview.height * 2), Image.NEAREST).save(sys.argv[1])
        return
    image.save(ROOT / "assets/textures/ingame/priest.tga")


if __name__ == "__main__":
    main()
