"""The temple's priests for the opening cinematic, drawn at low resolution: 10x19 pixel frames,
shown four times over (40x76, the hero's height) so their pixels are as chunky as the
dungeon's props. A hooded white robe with gold trim, the face in the hood's shadow.

Rows: 0 standing (four frames, the hem stirring), 1 running (four frames, side view, facing
right), 2 praying (two frames, hands raised, the second with a glint of light), 3 asleep on his
knees at prayer (two frames: hands fallen to his lap, the hood bowed over the face, sinking a
little further on the second).

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

# Running, side view facing right: a clean profile, the hood forward. The arm and the hem are
# drawn per frame (see run_frame).
RUN = [
    "....kkkk..",
    "...kwwwwk.",
    "..kwwwhhk.",
    "..kwwhffk.",
    "..kswhffk.",
    "..kswwhhk.",
    "..kgggggk.",
    ".kswwwwwk.",
    ".kswwwwwk.",
    ".kswwwwwk.",
    ".kswwwwwk.",
    ".kswwwwwk.",
    ".kswwwwwk.",
    ".kswwwwwk.",
    ".kswwwwwwk",
    ".kswwwwwwk",
    ".kGggggggk",
    "..........",
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


# Asleep at prayer: the hands have fallen, the hood bows over the face. Drawn from the standing
# robe, the head a pixel lower and its face gone into the hood's shadow.
def asleep(sink):
    rows = [list(line) for line in STAND]
    head = [row[:] for row in rows[:7]]
    for y in range(6 + sink, sink - 1, -1):
        rows[y] = head[y - sink][:] if y - sink >= 0 else list("..........")
    for y in range(sink):
        rows[y] = list("..........")
    for row in rows[:8]:
        for x, key in enumerate(row):
            if key == "f":
                row[x] = "h"
    # The hands, fallen into the lap.
    for y in (9, 10):
        rows[y][0] = "."
    rows[9][1] = "k"
    rows[13][4] = "F"
    rows[13][5] = "F"
    return draw(["".join(row) for row in rows])


def draw(rows, lift=0, sway=0, legs=None, glint=False):
    image = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    px = image.load()
    for y, line in enumerate(rows):
        line = line.ljust(W, ".")[:W]
        # The hem sways. (The head never moves on its own: at this size a head that rises
        # without the body looks as if it comes off.)
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


def run_frame(step):
    """One running frame: the arm swinging forward and back, the hem kicking, the feet
    stepping. `step` 0..3."""
    rows = [list(line) for line in RUN]
    # The arm: a hand out in front on 0 and 1, tucked back at the side on 2 and 3.
    if step in (0, 1):
        rows[9][8] = "F"
        rows[9][9] = "k"
    else:
        rows[10][1] = "F"
    # The hem kicks back as the leg goes forward.
    if step in (1, 3):
        for y in (14, 15, 16):
            rows[y] = list("k" + "".join(rows[y])[:-1]) if False else rows[y]
        rows[15] = list("kswwwwwwk.")
        rows[16] = list("kGggggggk.")
    image = draw(["".join(row) for row in rows])
    px = image.load()
    feet = {0: (3, 7), 1: (4, 6), 2: (6, 3), 3: (5, 4)}[step]
    for x in feet:
        for y in (H - 2, H - 1):
            px[x, y] = (*PALETTE["k"], 255)
            px[x + 1 if x + 1 < W else x, y] = (*PALETTE["k"], 255)
    return image


def sheet():
    out = Image.new("RGBA", (W * 4, H * 4), (0, 0, 0, 0))
    for i, lift in enumerate((0, 0, 1, 1)):
        out.alpha_composite(draw(STAND, sway=1 if i == 2 else 0), (i * W, 0))
    for i in range(4):
        out.alpha_composite(run_frame(i), (i * W, H))
    out.alpha_composite(draw(PRAY), (0, H * 2))
    out.alpha_composite(draw(PRAY, glint=True), (W, H * 2))
    out.alpha_composite(asleep(1), (0, H * 3))
    out.alpha_composite(asleep(2), (W, H * 3))
    return out


# The people of Velmor who come to pray: the same figure in a plain brown cloak and a dark
# undyed sash instead of the order's white and gold.
PILGRIM = {
    PALETTE["w"]: (150, 116, 84),
    PALETTE["s"]: (112, 84, 62),
    PALETTE["d"]: (78, 58, 46),
    PALETTE["g"]: (92, 74, 58),
    PALETTE["G"]: (64, 50, 40),
}


def pilgrim(image):
    out = image.copy()
    px = out.load()
    for y in range(out.height):
        for x in range(out.width):
            r, g, b, a = px[x, y]
            if a and (r, g, b) in PILGRIM:
                px[x, y] = (*PILGRIM[(r, g, b)], a)
    # No glint for the people: only the priests' prayers shine.
    for x, y in ((0, 1), (9, 1), (1, 0), (8, 0)):
        px[W + x, 2 * H + y] = (0, 0, 0, 0)
    return out


def main():
    image = sheet()
    if len(sys.argv) > 1:
        hero = Image.open(ROOT / "assets/textures/ingame/player.tga").convert("RGBA").crop((0, 0, 16, 38))
        big = image.resize((W * 4 * 4, H * 4 * 4), Image.NEAREST)
        preview = Image.new("RGBA", (big.width + 16 * 2 + 16, big.height), (30, 28, 36, 255))
        preview.alpha_composite(big, (0, 0))
        preview.alpha_composite(hero.resize((32, 76), Image.NEAREST), (big.width + 8, 0))
        preview.resize((preview.width * 2, preview.height * 2), Image.NEAREST).save(sys.argv[1])
        return
    image.save(ROOT / "assets/textures/ingame/priest.tga")
    pilgrim(image).save(ROOT / "assets/textures/ingame/pilgrim.tga")


if __name__ == "__main__":
    main()
