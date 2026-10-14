import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

OUTLINE = (48, 44, 46, 255)
STONE = (125, 112, 113, 255)
SHADE = (90, 83, 83, 255)
LIGHT = (160, 147, 142, 255)
CRYSTAL = (40, 204, 223, 255)
CRYSTAL_HOT = (138, 235, 241, 255)
CRYSTAL_DIM = (30, 120, 138, 255)
SIZE = 32


def poly(d, points, fill, dx=0, dy=0):
    d.polygon([(x + dx, y + dy) for x, y in points], fill=fill, outline=OUTLINE)


def facet(d, points, fill, dx=0, dy=0):
    d.polygon([(x + dx, y + dy) for x, y in points], fill=fill)


def hand(d, x, y):
    poly(d, [(x, y), (x + 4, y), (x + 4, y + 3), (x, y + 3)], STONE)
    for claw in (x, x + 2, x + 4):
        d.point((claw, y + 4), fill=OUTLINE)
    d.point((x + 1, y + 1), fill=LIGHT)


def golem(d, bob=0, sway=0, crystal=CRYSTAL, hot=CRYSTAL_HOT, tilt=0, parts=None):
    off = {name: (0, 0) for name in ("torso", "left", "right", "head", "lhand", "rhand")}
    off.update(parts or {})

    def at(name):
        return tilt + off[name][0], bob + off[name][1]

    dx, dy = at("torso")
    poly(d, [(10, 12), (22, 12), (21, 20), (16, 27), (11, 20)], STONE, dx, dy)
    facet(d, [(17, 13), (21, 13), (20, 20), (16, 25)], SHADE, dx, dy)
    facet(d, [(11, 14), (14, 13), (12, 19)], LIGHT, dx, dy)
    d.line([(11 + dx, 15 + dy), (20 + dx, 21 + dy)], fill=SHADE)
    dx, dy = at("left")
    poly(d, [(3, 11), (8, 8), (11, 12), (10, 19), (5, 17)], STONE, dx, dy)
    facet(d, [(4, 12), (8, 10), (8, 14), (5, 15)], LIGHT, dx, dy)
    dx, dy = at("right")
    poly(d, [(29, 11), (24, 8), (21, 12), (22, 19), (27, 17)], SHADE, dx, dy)
    facet(d, [(25, 10), (27, 12), (26, 15), (24, 13)], STONE, dx, dy)
    dx, dy = at("head")
    poly(d, [(16, 1), (20, 5), (19, 11), (13, 11), (12, 5)], SHADE, dx, dy)
    facet(d, [(13, 6), (15, 3), (14, 10)], STONE, dx, dy)
    d.line([(16 + dx, 4 + dy), (16 + dx, 9 + dy)], fill=crystal)
    d.point((16 + dx, 6 + dy), fill=hot)
    dx, dy = at("lhand")
    hand(d, 2 + dx, 20 + dy + sway)
    dx, dy = at("rhand")
    hand(d, 26 + dx, 20 + dy - sway)


def walk(frame):
    steps = [(0, 0), (1, 1), (0, 0), (1, -1)]
    bob, sway = steps[frame]
    image = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    golem(ImageDraw.Draw(image), bob, sway)
    return image


def chunk(d, x, y, w, h, fill=STONE):
    poly(d, [(x + 1, y), (x + w, y), (x + w, y + h - 1), (x + w - 1, y + h), (x, y + h), (x, y + 1)], fill)
    d.line([(x + 1, y + 1), (x + w - 2, y + 1)], fill=LIGHT)


def sunk(image, ground):
    pixels = image.load()
    for y in range(ground, SIZE):
        for x in range(SIZE):
            pixels[x, y] = (0, 0, 0, 0)
    return image


def death(frame):
    image = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(image)
    dim = dict(crystal=CRYSTAL_DIM, hot=CRYSTAL_DIM)
    if frame == 0:
        golem(d, tilt=1, crystal=CRYSTAL_HOT, hot=(255, 255, 255, 255))
    elif frame == 1:
        golem(d, bob=1, parts={"left": (-2, -1), "right": (2, -1), "lhand": (-2, 2), "rhand": (2, 2), "head": (0, 1)}, **dim)
    elif frame == 2:
        golem(d, bob=1, parts={"torso": (0, 3), "left": (-3, 4), "right": (3, 4), "lhand": (-3, 5), "rhand": (3, 5), "head": (0, 4)}, **dim)
        sunk(image, 31)
    elif frame == 3:
        chunk(d, 8, 23, 16, 6, SHADE)
        golem(d, parts={"torso": (0, 60), "left": (-2, 11), "right": (2, 11), "lhand": (0, 60), "rhand": (0, 60), "head": (0, 12)}, **dim)
    elif frame == 4:
        chunk(d, 8, 25, 16, 4, SHADE)
        chunk(d, 2, 25, 7, 4)
        chunk(d, 23, 25, 7, 4)
        golem(d, parts={"torso": (0, 60), "left": (0, 60), "right": (0, 60), "lhand": (0, 60), "rhand": (0, 60), "head": (0, 17)}, **dim)
    else:
        chunk(d, 4, 26, 8, 3)
        chunk(d, 11, 24, 10, 5, SHADE)
        chunk(d, 21, 26, 8, 3)
        d.line([(16, 22), (16, 23)], fill=CRYSTAL_DIM)
    return image


def main():
    sheet = Image.new("RGBA", (SIZE * 6, SIZE * 2), (0, 0, 0, 0))
    for frame in range(4):
        sheet.paste(walk(frame), (frame * SIZE, 0))
    for frame in range(6):
        sheet.paste(death(frame), (frame * SIZE, SIZE))
    sheet.save(OUT / "mini_golem.tga")


main()
