import math
import random
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

CLEAR = (0, 0, 0, 0)
ROCK_DARK = (37, 19, 26, 255)
ROCK = (74, 42, 40, 255)
ROCK_LIGHT = (112, 70, 58, 255)
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


def geyser_frames():
    width, height, count = 16, 32, 8
    sheet = Image.new("RGBA", (width * count, height), CLEAR)
    r = random.Random(3)

    def rim(image, glow):
        for y in range(24, 32):
            for x in range(16):
                dx, dy = (x - 7.5) / 7.5, (y - 28) / 4.2
                d = dx * dx + dy * dy
                if d < 1.0:
                    image.putpixel((x, y), ROCK_DARK if d > 0.78 else ROCK if (x + y) % 3 else ROCK_LIGHT)
        for y in range(26, 31):
            for x in range(3, 13):
                dx, dy = (x - 7.5) / 4.8, (y - 28.2) / 2.2
                if dx * dx + dy * dy < 1.0:
                    image.putpixel((x, y), glow)

    def flame(image, base, reach, half_width, wobble):
        for step in range(reach):
            y = base - step
            if y < 0:
                break
            ratio = step / max(reach, 1)
            width_here = max(1.0, half_width * (1.0 - ratio * 0.85))
            centre = 7.5 + wobble * math.sin(step * 0.5 + wobble)
            for x in range(16):
                distance = abs(x - centre)
                if distance < width_here:
                    inner = distance < width_here * 0.45
                    if ratio > 0.8 and r.random() < 0.4:
                        continue
                    colour = LAVA_CORE if inner and ratio < 0.7 else LAVA_HOT if inner or ratio < 0.5 else LAVA
                    if distance > width_here - 1.0 and ratio > 0.3:
                        colour = LAVA_DEEP
                    image.putpixel((x, y), colour)

    plan = [
        (LAVA_DEEP, 0, 0),
        (LAVA_DEEP, 0, 0),
        (LAVA, 3, 1.5),
        (LAVA_HOT, 6, 2.5),
        (LAVA_HOT, 14, 4.5),
        (LAVA_CORE, 26, 5.5),
        (LAVA_CORE, 28, 5.0),
        (LAVA, 12, 3.0),
    ]
    for frame, (glow, reach, half_width) in enumerate(plan):
        image = Image.new("RGBA", (width, height), CLEAR)
        rim(image, glow)
        if reach:
            flame(image, 28, reach, half_width, 0.8 + frame * 0.3)
            if frame in (5, 6):
                for _ in range(5):
                    image.putpixel((r.randrange(2, 14), r.randrange(0, 8)), LAVA_HOT)
        sheet.paste(image, (frame * width, 0))
    return sheet


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
    stone = (158, 142, 136, 255)
    light = (207, 194, 176, 255)
    shade = (125, 112, 113, 255)
    dark = (37, 19, 26, 255)
    for y in range(16):
        for x in range(16):
            image.putpixel((x, y), stone)
    for x in range(16):
        image.putpixel((x, 0), light)
        image.putpixel((x, 15), shade)
    for y in range(16):
        image.putpixel((0, y), light)
        image.putpixel((15, y), shade)
    for y in range(5, 14):
        for x in range(5, 11):
            image.putpixel((x, y), dark)
    for x in range(6, 10):
        image.putpixel((x, 5), (60, 36, 36, 255))
    for y in range(8, 13):
        image.putpixel((7, y), (120, 90, 60, 255))
        image.putpixel((8, y), (120, 90, 60, 255))
    image.putpixel((7, 13), (200, 200, 210, 255))
    image.putpixel((8, 13), (200, 200, 210, 255))
    for corner in ((2, 2), (13, 2), (2, 13), (13, 13)):
        image.putpixel(corner, shade)
    return image


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
    geyser_frames().save(OUT / "fire_geyser.tga")
    ice_patch().save(OUT / "ice_patch.tga")
    web_patch().save(OUT / "web_patch.tga")
    arrow_trap().save(OUT / "arrow_trap.tga")
    trap_arrow().save(OUT / "trap_arrow.tga")


main()
