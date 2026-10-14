import math
import random
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

CLEAR = (0, 0, 0, 0)
LAVA_DEEP = (176, 48, 22, 255)
LAVA = (222, 92, 28, 255)
LAVA_HOT = (255, 160, 48, 255)
LAVA_CORE = (255, 226, 120, 255)


def blob(size, seed, margin, wobble):
    r = random.Random(seed)
    phases = [(r.uniform(0, math.tau), r.uniform(0.6, 1.4)) for _ in range(3)]
    mask = [[False] * size for _ in range(size)]
    inside = {}
    centre = (size - 1) / 2
    radius = size / 2 - margin
    for y in range(size):
        for x in range(size):
            dx, dy = x - centre, y - centre
            angle = math.atan2(dy, dx)
            edge = radius + wobble * sum(math.sin(angle * (index + 2) * freq + phase) for index, (phase, freq) in enumerate(phases)) / 3
            distance = math.hypot(dx * 0.95, dy * 1.05)
            inside[(x, y)] = edge - distance
            mask[y][x] = distance < edge
    return mask, inside


def ice_patch():
    size = 32
    mask, depth = blob(size, 21, 2, 3.0)
    image = Image.new("RGBA", (size, size), CLEAR)
    r = random.Random(5)
    for y in range(size):
        for x in range(size):
            if not mask[y][x]:
                continue
            d = depth[(x, y)]
            if d < 1.4:
                colour = (120, 170, 210, 200)
            else:
                colour = (170, 214, 240, 190) if (x + y // 2) % 5 else (196, 232, 250, 190)
            image.putpixel((x, y), colour)
    for _ in range(5):
        x, y = r.randrange(7, 22), r.randrange(7, 22)
        for step in range(r.randint(3, 6)):
            if mask[min(31, y + step)][min(31, x + step)] and depth[(min(31, x + step), min(31, y + step))] > 2:
                image.putpixel((x + step, y + step), (255, 255, 255, 230))
    return image


def web_patch():
    size = 32
    image = Image.new("RGBA", (size, size), CLEAR)
    centre = (size - 1) / 2
    colour = (211, 212, 167, 215)
    soft = (145, 145, 115, 180)
    rays = 8
    for ray in range(rays):
        angle = ray * math.tau / rays + 0.2
        for step in range(0, 15):
            x = round(centre + math.cos(angle) * step)
            y = round(centre + math.sin(angle) * step)
            image.putpixel((x, y), colour if step % 2 == 0 or step < 4 else soft)
    for ring, reach in enumerate((4, 8, 12)):
        for index in range(rays):
            a0 = index * math.tau / rays + 0.2
            a1 = (index + 1) * math.tau / rays + 0.2
            for t in range(0, 7):
                angle = a0 + (a1 - a0) * t / 6
                sag = reach - 1.4 * math.sin(math.pi * t / 6)
                x = round(centre + math.cos(angle) * sag)
                y = round(centre + math.sin(angle) * sag)
                image.putpixel((x, y), soft)
    return image


def arrow_trap():
    image = Image.new("RGBA", (16, 16), CLEAR)
    dark = (22, 14, 18, 255)
    edge_light = (190, 176, 158, 255)
    edge_shade = (88, 74, 74, 255)
    for y in range(6, 11):
        for x in range(6, 10):
            image.putpixel((x, y), dark)
    for x in range(5, 11):
        image.putpixel((x, 5), edge_shade)
    for y in range(5, 12):
        image.putpixel((5, y), edge_shade)
    for x in range(5, 11):
        image.putpixel((x, 11), edge_light)
    for y in range(6, 12):
        image.putpixel((10, y), edge_light)
    return image


def arrow_hit():
    frames, size = 4, 16
    sheet = Image.new("RGBA", (size * frames, size), CLEAR)
    wood = (176, 130, 84, 255)
    wood_dark = (120, 84, 52, 255)
    metal = (214, 214, 224, 255)
    metal_dark = (150, 150, 168, 255)
    feather = (222, 222, 232, 255)

    def px(image, x, y, colour):
        if 0 <= x < size and 0 <= y < size:
            image.putpixel((x, y), colour)

    def head(image, x, y):
        px(image, x, y, metal)
        px(image, x - 1, y - 1, metal_dark)
        px(image, x + 1, y - 1, metal_dark)
        px(image, x, y - 1, metal)

    def shaft(image, x, y, length, shade=0):
        for step in range(length):
            px(image, x, y + step, wood if (step + shade) % 2 else wood_dark)

    f0 = Image.new("RGBA", (size, size), CLEAR)
    head(f0, 8, 12)
    shaft(f0, 8, 6, 5)
    px(f0, 7, 5, feather)
    px(f0, 9, 5, feather)
    px(f0, 6, 12, (255, 238, 190, 255))
    px(f0, 10, 12, (255, 238, 190, 255))

    f1 = Image.new("RGBA", (size, size), CLEAR)
    head(f1, 7, 13)
    shaft(f1, 7, 11, 2)
    shaft(f1, 9, 4, 4)
    px(f1, 8, 3, feather)
    px(f1, 10, 3, feather)
    px(f1, 5, 12, wood)
    px(f1, 11, 12, wood_dark)

    f2 = Image.new("RGBA", (size, size), CLEAR)
    head(f2, 5, 14)
    shaft(f2, 10, 9, 3)
    shaft(f2, 7, 12, 2, 1)
    px(f2, 11, 7, feather)
    px(f2, 13, 12, wood)
    px(f2, 3, 11, wood_dark)
    px(f2, 9, 13, wood)

    f3 = Image.new("RGBA", (size, size), CLEAR)
    px(f3, 4, 15, metal_dark)
    px(f3, 11, 14, wood_dark)
    px(f3, 8, 15, wood)
    px(f3, 14, 14, wood)
    px(f3, 2, 13, wood_dark)

    for frame, image in enumerate((f0, f1, f2, f3)):
        sheet.paste(image, (frame * size, 0))
    return sheet


def trap_arrow():
    image = Image.new("RGBA", (5, 14), CLEAR)
    shaft = (140, 98, 60, 255)
    shaft_light = (176, 130, 84, 255)
    head = (214, 214, 224, 255)
    head_dark = (150, 150, 168, 255)
    feather = (222, 222, 232, 255)
    for y in range(0, 10):
        image.putpixel((2, y), shaft if y % 2 else shaft_light)
    for y in (0, 1, 2):
        image.putpixel((1, y), feather)
        image.putpixel((3, y), feather)
    image.putpixel((0, 0), CLEAR)
    image.putpixel((2, 10), head)
    for x in (1, 2, 3):
        image.putpixel((x, 11), head if x == 2 else head_dark)
    for x in (0, 1, 2, 3, 4):
        image.putpixel((x, 11), head_dark if x in (0, 4) else head)
    image.putpixel((2, 12), head)
    image.putpixel((2, 13), head_dark)
    return image


def main():
    ice_patch().save(OUT / "ice_patch.tga")
    web_patch().save(OUT / "web_patch.tga")
    arrow_trap().save(OUT / "arrow_trap.tga")
    arrow_hit().save(OUT / "arrow_hit.tga")
    trap_arrow().save(OUT / "trap_arrow.tga")


main()
