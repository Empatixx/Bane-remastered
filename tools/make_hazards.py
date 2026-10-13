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


FLAME_RAMP = [
    (110, 28, 18, 255),
    (176, 48, 22, 255),
    (222, 92, 28, 255),
    (255, 140, 40, 255),
    (255, 190, 70, 255),
    (255, 232, 140, 255),
    (255, 250, 214, 255),
]
SMOKE = (70, 44, 44, 160)


def geyser_frames():
    width, height, count = 32, 64, 8
    sheet = Image.new("RGBA", (width * count, height), CLEAR)
    base_y = 56
    centre = (width - 1) / 2

    def rim(image, glow, hot):
        for y in range(base_y - 6, base_y + 7):
            for x in range(width):
                dx, dy = (x - centre) / 15.0, (y - base_y) / 6.0
                d = dx * dx + dy * dy
                if d < 1.0:
                    if d > 0.82:
                        colour = ROCK_DARK
                    elif d > 0.55:
                        colour = ROCK_LIGHT if (x * 3 + y * 5) % 7 < 2 else ROCK
                    else:
                        colour = ROCK
                    image.putpixel((x, y), colour)
        for y in range(base_y - 4, base_y + 5):
            for x in range(width):
                dx, dy = (x - centre) / 9.5, (y - base_y) / 3.6
                d = dx * dx + dy * dy
                if d < 1.0:
                    level = min(len(FLAME_RAMP) - 1, int((1.0 - d) * (2 + hot) + glow))
                    image.putpixel((x, y), FLAME_RAMP[max(0, level)])

    def flame(image, reach, half_width, phase, rng):
        for step in range(reach):
            y = base_y - 3 - step
            if y < 0:
                break
            ratio = step / reach
            sway = math.sin(step * 0.22 + phase) * (1.5 + 5.0 * ratio) + math.sin(step * 0.47 + phase * 2) * 1.2
            body = half_width * (1.0 - ratio ** 1.6 * 0.92)
            for x in range(width):
                distance = abs(x - centre - sway)
                if distance > body:
                    continue
                edge = distance / max(body, 0.5)
                heat = (1.0 - edge) * (1.0 - ratio * 0.55)
                heat += rng.uniform(-0.06, 0.06)
                if ratio > 0.82 and rng.random() < (ratio - 0.8) * 3.0:
                    continue
                level = max(0, min(len(FLAME_RAMP) - 1, int(heat * len(FLAME_RAMP) + 0.3)))
                image.putpixel((x, y), FLAME_RAMP[level])

    plan = [
        (0, 0, 0, 0.0),
        (0, 0, 0, 0.0),
        (1, 0, 0, 0.0),
        (2, 12, 6, 0.0),
        (3, 30, 10, 0.6),
        (3, 52, 12, 1.4),
        (3, 56, 11, 2.3),
        (2, 26, 7, 3.1),
    ]
    for frame, (glow, reach, half_width, phase) in enumerate(plan):
        rng = random.Random(40 + frame)
        image = Image.new("RGBA", (width, height), CLEAR)
        rim(image, glow, 1 if frame >= 3 else 0)
        if reach:
            flame(image, reach, half_width, phase, rng)
            for _ in range(4 + reach // 6):
                x = rng.randrange(6, 26)
                y = rng.randrange(max(0, base_y - reach - 6), base_y - 6)
                image.putpixel((x, y), FLAME_RAMP[rng.randint(3, 5)])
        if frame in (0, 1, 2):
            for _ in range(2):
                image.putpixel((rng.randrange(10, 22), base_y - 4 - rng.randrange(0, 6) - frame * 3), SMOKE)
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
    dark = (28, 18, 24, 255)
    deep = (14, 8, 12, 255)
    edge_light = (190, 176, 158, 255)
    edge_shade = (88, 74, 74, 255)
    for y in range(5, 14):
        for x in range(6, 10):
            image.putpixel((x, y), dark)
    for y in range(6, 13):
        image.putpixel((7, y), deep)
        image.putpixel((8, y), deep)
    for y in range(4, 15):
        image.putpixel((5, y), edge_shade)
    for x in range(5, 11):
        image.putpixel((x, 4), edge_shade)
    for x in range(5, 11):
        image.putpixel((x, 14), edge_light)
    for y in range(5, 15):
        image.putpixel((10, y), edge_light)
    image.putpixel((5, 14), edge_light)
    image.putpixel((10, 4), edge_shade)
    return image


def arrow_hit():
    frames, size = 6, 32
    sheet = Image.new("RGBA", (size * frames, size), CLEAR)
    centre = (size - 1) / 2
    wood = (176, 130, 84, 255)
    wood_dark = (120, 84, 52, 255)
    metal = (232, 232, 240, 255)
    for frame in range(frames):
        r = random.Random(900 + frame)
        image = Image.new("RGBA", (size, size), CLEAR)
        progress = frame / (frames - 1)
        radius = 3 + frame * 2.4
        for y in range(size):
            for x in range(size):
                dx, dy = x - centre, y - centre
                distance = math.hypot(dx, dy)
                angle = math.atan2(dy, dx)
                spikes = 1 + 0.45 * math.sin(angle * 7 + frame * 0.9) + 0.25 * math.sin(angle * 3 - frame)
                edge = radius * spikes
                if distance < edge:
                    heat = (1 - distance / max(edge, 1)) * (1 - progress * 0.75) + r.uniform(-0.08, 0.08)
                    if progress > 0.5 and r.random() < (progress - 0.5) * 1.6:
                        continue
                    level = max(0, min(len(FLAME_RAMP) - 1, int(heat * len(FLAME_RAMP) + 0.6)))
                    image.putpixel((x, y), FLAME_RAMP[level])
        for _ in range(4 + frame * 2):
            angle = r.uniform(0, math.tau)
            distance = radius * r.uniform(0.9, 1.6)
            x, y = int(centre + math.cos(angle) * distance), int(centre + math.sin(angle) * distance)
            if 0 <= x < size and 0 <= y < size:
                image.putpixel((x, y), r.choice([wood, wood_dark, metal, FLAME_RAMP[4]]))
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
    geyser_frames().save(OUT / "fire_geyser.tga")
    ice_patch().save(OUT / "ice_patch.tga")
    web_patch().save(OUT / "web_patch.tga")
    arrow_trap().save(OUT / "arrow_trap.tga")
    arrow_hit().save(OUT / "arrow_hit.tga")
    trap_arrow().save(OUT / "trap_arrow.tga")


main()
