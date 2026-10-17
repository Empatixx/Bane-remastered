"""Dresses the shopkeeper up to match the shop: gold eyes glowing out of the hood, a gold rim
round the hood's opening and a gold clasp under the chin. The green lantern in his hand goes,
and so does the flame he used to conjure: he only sways now, through his first three frames.

Works on shopkeeper.tga in place (ten 80x90 frames in a row). Each rule only touches the
robe's own colours, so running it again changes nothing. Its sway moved his arm in and out;
make_shopkeeper_idle.py now rebuilds the sheet from the rest pose (frame 0) instead.
"""

import sys
from pathlib import Path

from PIL import Image

PATH = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/shopkeeper.tga"
FRAME = (80, 90)

ROBE = (109, 24, 35, 255)
DARK = (25, 15, 32, 255)
EYE = (239, 236, 236, 255)
GLOW = (255, 214, 110, 255)
GOLD = (204, 160, 54, 255)
GOLD_D = (150, 106, 34, 255)
GOLD_H = (244, 214, 110, 255)
CLASP_BELOW_EYES = 5
LANTERN = {
    (17, 97, 22, 255), (12, 63, 15, 255), (15, 83, 20, 255), (126, 194, 98, 255),
    (99, 173, 69, 255), (133, 191, 19, 255), (103, 148, 15, 255),
}
# The swaying frames, played out over all ten; the first three stay where they were.
SWAY = [0, 1, 2, 2, 1, 0, 0, 1, 2, 1]


def unlantern(frame):
    """Paint the robe back over the lantern, from the robe round it."""
    px = frame.load()
    width, height = frame.size
    holes = {(x, y) for y in range(height) for x in range(width) if px[x, y] in LANTERN}
    while holes:
        filled = {}
        for x, y in holes:
            around = [px[x + dx, y + dy] for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1))
                      if 0 <= x + dx < width and 0 <= y + dy < height and (x + dx, y + dy) not in holes]
            around = [colour for colour in around if colour in (ROBE, DARK)]
            if around:
                filled[(x, y)] = max(set(around), key=around.count)
        if not filled:
            for x, y in holes:
                px[x, y] = ROBE
            break
        for (x, y), colour in filled.items():
            px[x, y] = colour
        holes -= set(filled)


def dress(frame):
    width, height = frame.size
    px = frame.load()
    whites = [(x, y) for y in range(height) for x in range(width) if px[x, y] in (EYE, GLOW)]
    if not whites:
        return
    # The eyes are the topmost light pixels; the rest are the hands and the bottle's cork.
    eye_y = min(y for _, y in whites)
    eyes = [(x, y) for x, y in whites if y == eye_y]
    left = min(x for x, _ in eyes)
    right = max(x for x, _ in eyes)
    for x, y in eyes:
        px[x, y] = GLOW
    # The rim of the hood: robe next to the dark inside of the hood, round the face.
    for y in range(max(1, eye_y - 4), eye_y + 4):
        for x in range(left - 3, right + 4):
            if px[x, y] != ROBE:
                continue
            around = [px[x + dx, y + dy] for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))]
            if DARK in around:
                px[x, y] = GOLD if y < eye_y + 2 else GOLD_D
    # A gold clasp holding the robe together under the chin.
    middle = (left + right) // 2
    for dx, dy, colour in ((0, 0, GOLD_H), (1, 0, GOLD), (0, 1, GOLD), (1, 1, GOLD_D)):
        x, y = middle + dx, eye_y + CLASP_BELOW_EYES + dy
        if px[x, y][3]:
            px[x, y] = colour


def main():
    sheet = Image.open(PATH).convert("RGBA")
    box = lambda index: (index * FRAME[0], 0, (index + 1) * FRAME[0], FRAME[1])
    sway = []
    for index in range(3):
        frame = sheet.crop(box(index))
        unlantern(frame)
        dress(frame)
        sway.append(frame)
    out = Image.new("RGBA", sheet.size, (0, 0, 0, 0))
    for index, source in enumerate(SWAY):
        out.paste(sway[source], box(index)[:2])
    out.save(PATH)


if __name__ == "__main__":
    main()
