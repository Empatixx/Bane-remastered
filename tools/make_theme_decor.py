import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
CLEAR = (0, 0, 0, 0)

ICE_WHITE = (236, 250, 255, 255)
ICE_LIGHT = (176, 218, 242, 255)
ICE_MID = (116, 166, 208, 255)
ICE_DARK = (66, 104, 156, 255)
ICE_DEEP = (36, 62, 108, 255)

BONE = (214, 208, 190, 255)
BONE_SHADE = (150, 142, 128, 255)
SOCKET = (28, 22, 30, 255)

SILK = (214, 214, 170, 255)
SILK_SHADE = (150, 150, 118, 255)
WRAP = (178, 176, 140, 255)
WRAP_DARK = (110, 108, 88, 255)
EGG = (226, 220, 190, 255)
EGG_SHADE = (160, 152, 124, 255)
EGG_SPOT = (110, 130, 70, 255)


def sprite(rows, palette):
    image = Image.new("RGBA", (len(rows[0]), len(rows)), CLEAR)
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key in palette:
                image.putpixel((x, y), palette[key])
    return image


def icicles(seed):
    r = random.Random(seed)
    image = Image.new("RGBA", (16, 16), CLEAR)
    for x in range(16):
        image.putpixel((x, 0), ICE_LIGHT if x % 3 else ICE_WHITE)
        image.putpixel((x, 1), ICE_MID if x % 2 else ICE_LIGHT)
    centres = sorted(r.sample(range(1, 15), 5))
    last = -9
    for centre in centres:
        if centre - last < 3:
            continue
        last = centre
        length = r.randint(5, 13)
        for y in range(2, 2 + length):
            half = max(0.0, 1.6 * (1 - (y - 2) / length))
            for x in range(16):
                distance = x - centre
                if abs(distance) <= half or (half < 0.6 and distance == 0):
                    colour = ICE_WHITE if distance < 0 and y % 3 else ICE_LIGHT if distance <= 0 else ICE_MID
                    image.putpixel((x, y), colour)
    return image


def crystals(seed):
    r = random.Random(seed)
    image = Image.new("RGBA", (16, 16), CLEAR)
    shards = [(r.randint(3, 6), 14, r.randint(7, 10), 2.2), (r.randint(8, 10), 14, r.randint(9, 13), 2.6), (r.randint(11, 13), 14, r.randint(5, 8), 2.0)]
    draw = ImageDraw.Draw(image)
    for centre, base, height, width in shards:
        top = base - height
        tilt = r.choice([-1, 0, 1])
        points = [(centre - width, base), (centre - width * 0.6 + tilt, top + 2), (centre + tilt, top), (centre + width * 0.6 + tilt, top + 2), (centre + width, base)]
        draw.polygon(points, fill=ICE_MID)
        draw.polygon([(centre - width, base), (centre - width * 0.6 + tilt, top + 2), (centre + tilt, top), (centre, base)], fill=ICE_LIGHT)
        draw.line([(centre - 1, base - 1), (centre - 1 + tilt, top + 3)], fill=ICE_WHITE)
        draw.line([(centre + width - 1, base), (centre + width * 0.6 + tilt, top + 2)], fill=ICE_DARK)
    for x in range(1, 15):
        if image.getpixel((x, 14))[3]:
            image.putpixel((x, 15), ICE_DEEP)
    return image


SKULL = [
    "....wwwwww....",
    "..wwwwwwwwww..",
    ".wwwwwwwwwwww.",
    ".wwwwwwwwwwww.",
    ".wkkkwwwwkkkw.",
    ".wkkkwwwwkkkw.",
    ".wwwwwkkwwwww.",
    "..wwwwkkwwww..",
    "...wwwwwwww...",
    "...wswswsww...",
    "....wsssww....",
]


def skull_pixels():
    palette = {"w": BONE, "s": BONE_SHADE, "k": SOCKET}
    return sprite(SKULL, palette)


def frozen_skull():
    image = Image.new("RGBA", (16, 16), CLEAR)
    draw = ImageDraw.Draw(image)
    draw.rectangle((0, 3, 15, 15), fill=ICE_MID)
    draw.rectangle((0, 3, 15, 4), fill=ICE_LIGHT)
    draw.rectangle((0, 3, 1, 15), fill=ICE_LIGHT)
    draw.rectangle((14, 5, 15, 15), fill=ICE_DARK)
    draw.rectangle((0, 14, 15, 15), fill=ICE_DEEP)
    skull = skull_pixels()
    skull = skull.resize((11, 10), Image.NEAREST)
    cloudy = Image.new("RGBA", skull.size, CLEAR)
    for y in range(skull.height):
        for x in range(skull.width):
            pixel = skull.getpixel((x, y))
            if pixel[3]:
                tint = (int(pixel[0] * 0.62 + ICE_MID[0] * 0.38), int(pixel[1] * 0.66 + ICE_MID[1] * 0.34), int(pixel[2] * 0.74 + ICE_MID[2] * 0.26), 255)
                cloudy.putpixel((x, y), tint)
    image.alpha_composite(cloudy, (2, 4))
    for x, y in ((12, 6), (11, 9), (13, 11), (4, 12)):
        image.putpixel((x, y), ICE_WHITE)
    return image


OUTLINE = (52, 48, 36, 255)
SHADOW = (14, 10, 18, 110)


def shadow_ellipse(image, box):
    layer = Image.new("RGBA", image.size, CLEAR)
    ImageDraw.Draw(layer).ellipse(box, fill=SHADOW)
    image.alpha_composite(layer)


def cocoon(variant):
    image = Image.new("RGBA", (16, 16), CLEAR)
    horizontal = variant == 1
    if horizontal:
        shadow_ellipse(image, (1, 8, 15, 15))
        body = (1, 3, 14, 12)
    else:
        shadow_ellipse(image, (3, 6, 14, 15))
        body = (4, 1, 12, 13)
    draw = ImageDraw.Draw(image)
    draw.ellipse(body, fill=WRAP, outline=OUTLINE)
    x0, y0, x1, y1 = body
    if horizontal:
        for x in range(x0 + 3, x1 - 1, 3):
            draw.line([(x, y0 + 1), (x + 1, y1 - 1)], fill=WRAP_DARK)
        draw.line([(x0 + 2, y0 + 2), (x1 - 3, y0 + 2)], fill=SILK)
        for start, end in (((0, 7), (x0, 7)), ((15, 7), (x1, 7)), ((1, 3), (x0 + 1, y0 + 1))):
            draw.line([start, end], fill=SILK_SHADE)
    else:
        for y in range(y0 + 3, y1 - 1, 3):
            draw.line([(x0 + 1, y), (x1 - 1, y + 1)], fill=WRAP_DARK)
        draw.line([(x0 + 2, y0 + 2), (x0 + 2, y1 - 3)], fill=SILK)
        for start, end in (((8, 0), (8, y0)), ((7, 15), (8, y1)), ((2, 12), (x0 + 1, y1 - 2))):
            draw.line([start, end], fill=SILK_SHADE)
    return image


EGG_SHELL = (232, 226, 190, 255)
EGG_LIGHT = (255, 252, 230, 255)
EGG_MID = (206, 198, 160, 255)
EGG_DARK = (150, 142, 108, 255)
GOO = (122, 156, 64, 255)
GOO_LIGHT = (176, 206, 98, 255)
GOO_DARK = (76, 104, 44, 255)
SIZE = 48
EGGS = [(15, 20, 7), (30, 17, 7), (24, 30, 8), (11, 33, 6), (36, 33, 6), (23, 11, 5)]


def egg(image, cx, cy, radius, cracked=0):
    draw = ImageDraw.Draw(image)
    layer = Image.new("RGBA", image.size, CLEAR)
    ImageDraw.Draw(layer).ellipse((cx - radius + 2, cy - radius + 3, cx + radius + 3, cy + radius + 4), fill=SHADOW)
    image.alpha_composite(layer)
    draw.ellipse((cx - radius, cy - radius, cx + radius, cy + radius), fill=EGG_SHELL, outline=OUTLINE)
    draw.ellipse((cx - radius + 2, cy - radius + 2, cx + radius - 1, cy + radius - 1), fill=EGG_MID)
    draw.ellipse((cx - radius + 2, cy - radius + 2, cx + 1, cy + 1), fill=EGG_SHELL)
    draw.rectangle((cx - radius + 3, cy - radius + 3, cx - radius + 4, cy - radius + 4), fill=EGG_LIGHT)
    draw.point((cx + radius - 3, cy + radius - 3), fill=EGG_DARK)
    for step in range(cracked):
        draw.point((cx + step - 2, cy - 3 + step * 2 % 5), fill=OUTLINE)
        draw.point((cx + step - 1, cy - 2 + step * 2 % 5), fill=OUTLINE)
    spot = (cx + 1, cy + 1)
    draw.point(spot, fill=EGG_SPOT)


def web_base(image):
    draw = ImageDraw.Draw(image)
    draw.ellipse((2, 8, 45, 44), fill=(214, 214, 170, 60))
    for angle in range(0, 360, 30):
        end = (24 + 22 * math.cos(math.radians(angle)), 26 + 18 * math.sin(math.radians(angle)))
        draw.line([(24, 26), end], fill=SILK_SHADE)
    for radius_x, radius_y in ((9, 7), (16, 12), (22, 17)):
        draw.arc((24 - radius_x, 26 - radius_y, 24 + radius_x, 26 + radius_y), 0, 360, fill=(150, 150, 118, 170))


def intact():
    image = Image.new("RGBA", (SIZE, SIZE), CLEAR)
    web_base(image)
    for cx, cy, radius in sorted(EGGS, key=lambda e: e[1]):
        egg(image, cx, cy, radius)
    return image


def goo_splash(image, spread, density, rng):
    draw = ImageDraw.Draw(image)
    for _ in range(density):
        angle = rng.uniform(0, math.tau)
        distance = rng.uniform(2, spread)
        x, y = 24 + math.cos(angle) * distance, 26 + math.sin(angle) * distance * 0.8
        size = rng.choice([1, 1, 2, 2, 3])
        colour = rng.choice([GOO, GOO, GOO_LIGHT, GOO_DARK])
        draw.ellipse((x - size, y - size, x + size, y + size), fill=colour)


def breaking(stage):
    rng = random.Random(500 + stage)
    image = Image.new("RGBA", (SIZE, SIZE), CLEAR)
    web_base(image)
    if stage == 0:
        for index, (cx, cy, radius) in enumerate(sorted(EGGS, key=lambda e: e[1])):
            egg(image, cx + rng.choice([-1, 0, 1]), cy, radius, cracked=3 + index % 2)
        return image
    if stage == 1:
        goo_splash(image, 15, 22, rng)
        for cx, cy, radius in EGGS:
            egg(image, cx + (cx - 24) // 4, cy + (cy - 26) // 4, max(3, radius - 3), cracked=4)
        return image
    goo_splash(image, 20 if stage == 2 else 18, 30 if stage == 2 else 36, rng)
    draw = ImageDraw.Draw(image)
    for cx, cy, radius in EGGS:
        for _ in range(3 if stage == 2 else 2):
            x = cx + rng.randint(-radius - 3, radius + 3) * (1 if stage == 2 else 1)
            y = cy + rng.randint(-radius, radius + 2)
            draw.polygon([(x, y), (x + rng.randint(2, 4), y + 1), (x + 1, y + rng.randint(2, 4))], fill=EGG_SHELL, outline=OUTLINE)
    if stage == 3:
        draw.ellipse((9, 21, 39, 38), fill=GOO_DARK)
        draw.ellipse((11, 22, 37, 36), fill=GOO)
        draw.ellipse((15, 24, 25, 29), fill=GOO_LIGHT)
        for _ in range(8):
            x, y = rng.randint(10, 36), rng.randint(22, 36)
            draw.polygon([(x, y), (x + 3, y), (x + 1, y + 2)], fill=EGG_SHELL, outline=OUTLINE)
    return image


def egg_sheet():
    sheet = Image.new("RGBA", (SIZE * 4, SIZE * 2), CLEAR)
    sheet.paste(intact(), (0, 0))
    for stage in range(4):
        sheet.paste(breaking(stage), (stage * SIZE, SIZE))
    return sheet


def stack(images):
    sheet = Image.new("RGBA", (16, 16 * len(images)), CLEAR)
    for index, image in enumerate(images):
        sheet.paste(image, (0, index * 16))
    return sheet


def main():
    stack([icicles(1), icicles(2), icicles(3)]).save(OUT / "icicles.tga")
    stack([crystals(4), crystals(5)]).save(OUT / "ice_crystals.tga")
    frozen_skull().save(OUT / "frozen_skull.tga")
    stack([cocoon(0), cocoon(1)]).save(OUT / "cocoon.tga")
    egg_sheet().save(OUT / "egg_sac.tga")


main()
