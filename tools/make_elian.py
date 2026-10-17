"""Elian, the novice the player plays, kneeling at his prayers, for the opening cinematic: drawn
from the hero's own back frame (player.tga, row 3), seen from behind as he faces the altar, at
the hero's pixel scale (13x31 cells).

Rows (4 frames each):
  0  kneeling at prayer, breathing (the whole body rises and falls together)
  1  nodding off: his head dips, jerks up, then drops as he falls asleep
  2  asleep on his knees, breathing

Kneeling: the upper body comes down three or four pixels, the legs fold under him so only the soles of
his boots show behind, and his hands, joined in front of him, are hidden by his back.

    uv run -q --with pillow python tools/make_elian.py [preview.png]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
W, H = 13, 31
OUTLINE = (30, 16, 20, 255)
TROUSERS = [(62, 68, 82, 255), (46, 50, 62, 255), (32, 34, 44, 255)]
SOLE = [(104, 64, 44, 255), (50, 28, 22, 255)]
HEAD_ROWS = range(0, 16)
TORSO_ROWS = range(16, 25)  # shoulders, arms, belt
# The hands at the ends of his arms (the lowest arm rows), hidden once they are joined in prayer.
HANDS = [(1, 20), (2, 20), (1, 21), (2, 21), (10, 20), (10, 21), (11, 20), (11, 21)]


def back_frame():
    sheet = Image.open(ROOT / "assets/textures/ingame/player.tga").convert("RGBA")
    return sheet.crop((0, 3 * H, W, 4 * H))


def kneel(body_drop, head_drop):
    """Elian kneeling, his upper body `body_drop` pixels down and his head a further
    `head_drop`, its hair falling over his shoulders as it bows."""
    source = back_frame()
    src = source.load()
    out = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    px = out.load()
    for y in TORSO_ROWS:
        for x in range(W):
            if (x, y) in HANDS:
                continue
            colour = src[x, y]
            ty = y + body_drop
            if colour[3] and ty < H:
                px[x, ty] = colour
    # Arms bent forward: close the sleeves where the hands were.
    for x, y in ((1, 20), (11, 20)):
        if y + body_drop < H:
            px[x, y + body_drop] = OUTLINE
    for y in HEAD_ROWS:
        for x in range(W):
            colour = src[x, y]
            ty = y + body_drop + head_drop
            if colour[3] and 0 <= ty < H:
                px[x, ty] = colour
    # The folded legs: a band of trousers over his heels, the soles of his boots behind.
    belt = 24 + body_drop
    for y in range(belt + 1, H):
        for x in range(W):
            px[x, y] = (0, 0, 0, 0)
    # Drawn up from the bottom, so the soles always show whatever the lean.
    soles = H - 1
    for x in (2, 3, 4, 8, 9, 10):
        px[x, soles] = OUTLINE if x in (2, 10) else SOLE[0 if x in (3, 9) else 1]
    for y in range(belt + 1, soles):
        shade = TROUSERS[0] if y == belt + 1 else TROUSERS[2]
        for x in range(2, 11):
            px[x, y] = OUTLINE if x in (2, 10) or (y > belt + 1 and x == 6) else shade
    return out


ROWS = [
    [(3, 0), (3, 0), (4, 0), (4, 0)],
    [(3, 1), (3, 0), (4, 2), (4, 2)],
    [(4, 2), (4, 2), (4, 3), (4, 3)],
]


def sheet():
    out = Image.new("RGBA", (W * 4, H * len(ROWS)), (0, 0, 0, 0))
    for row, frames in enumerate(ROWS):
        for column, (body, head) in enumerate(frames):
            out.alpha_composite(kneel(body, head), (column * W, row * H))
    return out


def main():
    image = sheet()
    if len(sys.argv) > 1:
        preview = Image.new("RGBA", (image.width + W + 4, image.height), (30, 28, 36, 255))
        preview.alpha_composite(image, (0, 0))
        preview.alpha_composite(back_frame(), (image.width + 4, 0))
        preview.resize((preview.width * 10, preview.height * 10), Image.NEAREST).save(sys.argv[1])
        return
    image.save(ROOT / "assets/textures/ingame/elian.tga")


if __name__ == "__main__":
    main()
