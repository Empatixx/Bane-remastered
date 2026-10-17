"""A workbench that can be knocked over for cover, one look per floor: standing, fallen away from
the camera (its top standing up towards us as a plate), fallen towards the camera (showing its
underside and legs), and broken into scrap.

table.tga has a row per floor, in the order of `Theme::style`, of nine 32x32 cells: those four,
then the same bench turned upright (standing lengthwise down the screen), that upright bench
fallen over to the right, its top facing left (the game mirrors it for a fall to the left), and
three frames of it bursting apart, bits flying out and settling into the scrap of the fourth
cell. The looks: worn steel with a little rust (stone), verdigris copper with brass rivets and
slime (sewer), planks grown over with moss (moss), dark wood hung with webs (spider), frosted
steel (ice), blackened iron with ember-bright cracks (lava) and red wood studded with gold
(temple). Drawn at the tiles' pixel scale.
"""

import random

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
S = 32
CLEAR = (0, 0, 0, 0)

D = (24, 26, 32)
M = (62, 66, 78)
L = (92, 98, 112)
H = (138, 144, 158)
SHADE = (46, 50, 60)
# Rivet heads.
N = (172, 176, 188)
# Rust.
R = (112, 64, 42)
r = (86, 52, 38)
# The floor in the bench's shade.
SHADOW = (6, 6, 10)


def blank():
    return Image.new("RGBA", (S, S), CLEAR)


def put(image, x, y, colour):
    if 0 <= x < S and 0 <= y < S:
        image.putpixel((x, y), colour if len(colour) == 4 else colour + (255,))


def rect(image, x0, y0, x1, y1, colour):
    for y in range(y0, y1 + 1):
        for x in range(x0, x1 + 1):
            put(image, x, y, colour)


def shadow(image, x0, y0, x1, y1, alpha):
    """Darken the floor showing round the bench, never the bench itself."""
    for y in range(y0, y1 + 1):
        for x in range(x0, x1 + 1):
            if 0 <= x < S and 0 <= y < S and image.getpixel((x, y))[3] == 0:
                image.putpixel((x, y), SHADOW + (alpha,))


def feet(image, xs, y):
    """A small dark pool where each leg meets the floor."""
    for x in xs:
        shadow(image, x - 1, y, x + 3, y, 170)
        shadow(image, x, y + 1, x + 2, y + 1, 110)


def rivet(image, x, y):
    put(image, x, y, N)
    put(image, x + 1, y + 1, SHADE)


def plate(image, x0, y0, x1, y1, seams=(), columns=(), rust=()):
    """A steel surface: plates with dark seams between them, rivets along each seam and in the
    corners, and a few spots of rust."""
    rect(image, x0, y0, x1, y1, L)
    rect(image, x0, y0, x1, y0, H)
    for y in seams:
        rect(image, x0, y, x1, y, SHADE)
        rect(image, x0, y + 1, x1, y + 1, H)
    for x in columns:
        rect(image, x, y0, x, y1, SHADE)
    for x, y in rust:
        put(image, x, y, R)
        put(image, x + 1, y, r)
        put(image, x, y + 1, r)
    for y in (y0 + 2, y1 - 2):
        rivet(image, x0 + 2, y)
        rivet(image, x1 - 3, y)
    for x in (x0, x1):
        rect(image, x, y0, x, y1, D)
    rect(image, x0, y1, x1, y1, D)


def leg(image, x, y0, y1):
    """A square iron leg seen from the front, lit on its left."""
    rect(image, x, y0, x + 2, y1, M)
    rect(image, x + 1, y0, x + 1, y1, L)
    rect(image, x, y0, x, y1, D)
    rect(image, x + 2, y0, x + 2, y1, D)
    rect(image, x, y1, x + 2, y1, D)


def standing():
    image = blank()
    plate(image, 1, 7, 30, 19, seams=(13,), rust=((20, 9), (8, 16)))
    rect(image, 1, 6, 30, 6, D)
    # Front rail and legs.
    rect(image, 2, 20, 29, 22, M)
    rect(image, 2, 20, 29, 20, SHADE)
    rect(image, 2, 22, 29, 22, D)
    for x in (6, 14, 22):
        rivet(image, x, 21)
    for x in (3, 26):
        leg(image, x, 23, 29)
    # Under the top between the legs, fading towards us.
    shadow(image, 6, 23, 25, 25, 150)
    shadow(image, 6, 26, 25, 27, 100)
    shadow(image, 7, 28, 24, 28, 55)
    feet(image, (3, 26), 30)
    return image


def fallen_away():
    """Tipped over away from the viewer: the top is now a wall of plate facing us."""
    image = blank()
    # Legs sticking up behind the plate.
    for x in (4, 25):
        leg(image, x, 3, 10)
        rect(image, x, 3, x + 2, 3, D)
    plate(image, 1, 10, 30, 27, seams=(18,), rust=((9, 14), (23, 22), (15, 25)))
    rect(image, 1, 9, 30, 9, D)
    shadow(image, 1, 28, 30, 28, 160)
    shadow(image, 2, 29, 29, 29, 90)
    return image


def fallen_towards():
    """Tipped over towards the viewer: the underside, a frame with a brace, legs pointing at us."""
    image = blank()
    rect(image, 1, 6, 30, 17, SHADE)
    rect(image, 1, 6, 30, 6, D)
    rect(image, 1, 17, 30, 17, D)
    rect(image, 1, 6, 1, 17, D)
    rect(image, 30, 6, 30, 17, D)
    # The angle-iron frame and a cross brace.
    rect(image, 3, 8, 28, 9, M)
    rect(image, 3, 14, 28, 15, M)
    rect(image, 15, 8, 16, 15, M)
    rect(image, 3, 8, 28, 8, L)
    for x in (4, 14, 17, 27):
        put(image, x, 9, N)
        put(image, x, 15, N)
    put(image, 10, 12, R)
    put(image, 21, 11, r)
    # Legs coming out at us, wider at the feet.
    for x in (2, 26):
        rect(image, x, 15, x + 3, 28, L)
        rect(image, x + 1, 15, x + 1, 28, H)
        rect(image, x, 15, x, 28, D)
        rect(image, x + 3, 15, x + 3, 28, D)
        rect(image, x, 28, x + 3, 29, D)
    shadow(image, 4, 18, 27, 19, 140)
    shadow(image, 4, 20, 27, 20, 80)
    for x in (2, 26):
        shadow(image, x - 1, 30, x + 4, 30, 160)
    return image


def broken():
    """Scrap: bent bits of plate and a snapped leg."""
    image = blank()
    pieces = [(3, 18, 13, 20), (15, 14, 26, 16), (8, 23, 20, 25), (21, 21, 28, 22)]
    for x0, y0, x1, y1 in pieces:
        rect(image, x0, y0, x1, y1, L)
        rect(image, x0, y0, x1, y0, H)
        rect(image, x0, y1, x1, y1, D)
        put(image, x1, y0, D)
        put(image, x0 + 1, y0 + 1, N)
    # A leg lying on its side.
    rect(image, 5, 12, 11, 13, M)
    rect(image, 5, 12, 11, 12, L)
    rect(image, 5, 14, 11, 14, D)
    for x, y in ((12, 19), (24, 15), (17, 24), (26, 22)):
        put(image, x, y, R)
    for x, y in ((18, 11), (26, 26), (10, 27)):
        put(image, x, y, M)
    return image


def upright():
    """Standing lengthwise down the screen: a narrow top two tiles long, legs at the front."""
    image = blank()
    plate(image, 8, 1, 23, 25, seams=(12,), rust=((17, 6), (11, 19)))
    rect(image, 9, 26, 22, 27, M)
    rect(image, 9, 27, 22, 27, D)
    for x in (9, 20):
        leg(image, x, 28, 31)
    shadow(image, 12, 28, 19, 29, 140)
    shadow(image, 12, 30, 19, 31, 90)
    shadow(image, 24, 3, 24, 26, 90)
    return image


def upright_fallen_right():
    """The upright bench tipped over to the right: its top is a tall plate facing left, seen
    edge-on from above, with the legs sticking out to the right behind it."""
    image = blank()
    for y in (4, 22):
        rect(image, 17, y, 25, y + 2, M)
        rect(image, 17, y, 25, y, L)
        rect(image, 25, y, 25, y + 2, D)
        rect(image, 17, y + 2, 25, y + 2, D)
    # The plate: a strip of its edge on top, its face towards the left.
    rect(image, 10, 1, 16, 29, L)
    rect(image, 10, 1, 16, 1, H)
    rect(image, 15, 1, 16, 29, SHADE)
    rect(image, 10, 15, 14, 15, SHADE)
    rect(image, 10, 16, 14, 16, H)
    for y in (3, 13, 18, 26):
        rivet(image, 11, y)
    put(image, 13, 8, R)
    put(image, 12, 22, r)
    rect(image, 9, 1, 9, 29, D)
    rect(image, 17, 1, 17, 29, D)
    rect(image, 9, 29, 17, 30, D)
    shadow(image, 18, 2, 19, 29, 140)
    shadow(image, 20, 2, 20, 29, 80)
    shadow(image, 9, 31, 17, 31, 150)
    return image


def burst(frame, seed):
    """The bench bursting apart: chunks of it flying out from the middle, further and fewer each
    frame, over the scrap they leave."""
    import math

    image = broken() if frame == 2 else blank()
    rng = random.Random(seed)
    reach = (7, 12, 15)[frame]
    count = (14, 11, 6)[frame]
    for _ in range(count):
        angle = rng.uniform(0, math.tau)
        distance = rng.uniform(reach * 0.45, reach)
        x = int(15 + distance * 1.3 * math.cos(angle))
        y = int(17 + distance * math.sin(angle))
        width = rng.choice((2, 3, 4)) if frame < 2 else rng.choice((1, 2))
        height = 2 if frame < 2 else 1
        rect(image, x, y, x + width - 1, y + height - 1, rng.choice((L, M, L, H)))
        rect(image, x, y + height, x + width - 1, y + height, D)
        if rng.random() < 0.3:
            put(image, x + width - 1, y, R)
    return image


# Each floor's look: the bench's darkest edge, its body, light and highlight, the shade in its
# seams, rivet (or nail, or stud) heads, and its two "rust" colours, which are whatever grows
# or glows on it there.
LOOKS = [
    ((24, 26, 32), (62, 66, 78), (92, 98, 112), (138, 144, 158), (46, 50, 60), (172, 176, 188), (112, 64, 42), (86, 52, 38)),
    ((18, 30, 26), (52, 84, 70), (76, 118, 98), (112, 156, 130), (38, 62, 52), (196, 156, 92), (88, 140, 64), (62, 104, 46)),
    ((34, 24, 16), (84, 60, 36), (112, 84, 50), (146, 112, 68), (66, 48, 30), (60, 56, 50), (92, 134, 52), (66, 100, 40)),
    ((16, 10, 14), (52, 38, 40), (74, 56, 56), (100, 80, 78), (40, 28, 30), (150, 150, 160), (214, 212, 196), (164, 162, 146)),
    ((18, 30, 48), (70, 96, 128), (110, 140, 172), (176, 206, 232), (52, 74, 104), (232, 242, 252), (224, 240, 252), (182, 212, 238)),
    ((14, 10, 10), (40, 32, 32), (60, 48, 46), (90, 74, 70), (30, 24, 24), (124, 104, 92), (255, 140, 50), (204, 80, 30)),
    ((40, 20, 14), (110, 62, 30), (146, 88, 44), (190, 128, 64), (84, 46, 24), (238, 198, 92), (228, 184, 82), (172, 130, 52)),
]


def look(index):
    global D, M, L, H, SHADE, N, R, r
    D, M, L, H, SHADE, N, R, r = LOOKS[index]


def main():
    sheet = Image.new("RGBA", (9 * S, len(LOOKS) * S), CLEAR)
    for row in range(len(LOOKS)):
        look(row)
        cells = [standing(), fallen_away(), fallen_towards(), broken(), upright(), upright_fallen_right()]
        cells += [burst(frame, row * 7 + frame) for frame in range(3)]
        for column, image in enumerate(cells):
            sheet.paste(image, (column * S, row * S))
    sheet.save(OUT / "table.tga")


if __name__ == "__main__":
    main()
