import colorsys
import math
from PIL import Image, ImageDraw

SRC = "assets/textures/ingame/barrel.tga"
OUT = "assets/textures/ingame"
FRAME = 64
METAL_SATURATION = 0.3


def recolor(image, hue, saturation, value_scale, value_add):
    out = image.copy()
    pixels = out.load()
    for y in range(out.height):
        for x in range(out.width):
            r, g, b, a = pixels[x, y]
            if a == 0:
                continue
            h, s, v = colorsys.rgb_to_hsv(r / 255, g / 255, b / 255)
            if s < METAL_SATURATION:
                continue
            s = min(1.0, s * saturation[0] + saturation[1])
            v = min(1.0, v * value_scale + value_add)
            r, g, b = colorsys.hsv_to_rgb(hue, s, v)
            pixels[x, y] = (int(r * 255), int(g * 255), int(b * 255), a)
    return out


def frost_speckles(image):
    pixels = image.load()
    for row in range(image.height // FRAME):
        for column in range(image.width // FRAME):
            for y in range(FRAME // 2):
                for x in range(FRAME):
                    px, py = column * FRAME + x, row * FRAME + y
                    r, g, b, a = pixels[px, py]
                    if a == 0:
                        continue
                    if ((x * 73856093) ^ (y * 19349663) ^ (column * 83492791)) % 19 == 0:
                        pixels[px, py] = (235, 248, 255, a)
    return image


def poison_marks(image):
    pixels = image.load()
    for row in range(image.height // FRAME):
        for column in range(image.width // FRAME):
            for y in range(FRAME // 3, FRAME):
                for x in range(FRAME):
                    px, py = column * FRAME + x, row * FRAME + y
                    r, g, b, a = pixels[px, py]
                    if a == 0:
                        continue
                    if ((x * 73856093) ^ (y * 19349663) ^ (column * 83492791)) % 31 == 0:
                        pixels[px, py] = TOXIC
    return image


def frost_burst():
    frames = 9
    sheet = Image.new("RGBA", (FRAME * frames, FRAME), (0, 0, 0, 0))
    for frame in range(frames):
        t = frame / (frames - 1)
        layer = Image.new("RGBA", (FRAME, FRAME), (0, 0, 0, 0))
        draw = ImageDraw.Draw(layer)
        fade = max(0.0, 1.0 - t * t * 0.95)
        glow = max(0, int(255 * (1 - t) * 0.9))
        radius = 4 + t * 26
        if t < 0.7:
            r = int(14 * (1 - t / 0.7)) + 2
            draw.ellipse((32 - r, 32 - r, 32 + r, 32 + r), fill=(200, 235, 255, glow))
        for ring, length, width, tone in ((radius, 8, 3, (230, 248, 255)), (radius * 0.6, 5, 2, (150, 215, 255))):
            count = 10 if ring == radius else 8
            offset = 0.2 if ring == radius else 0.55
            for i in range(count):
                angle = (i / count) * math.tau + offset
                ux, uy = math.cos(angle), math.sin(angle)
                px, py = -uy, ux
                tip = (32 + ux * (ring + length), 32 + uy * (ring + length))
                tail = (32 + ux * (ring - length * 0.6), 32 + uy * (ring - length * 0.6))
                mid = (32 + ux * ring, 32 + uy * ring)
                poly = [tip, (mid[0] + px * width, mid[1] + py * width), tail, (mid[0] - px * width, mid[1] - py * width)]
                draw.polygon(poly, fill=tone + (int(255 * fade),))
        for i in range(6):
            angle = i * 1.7 + frame * 0.6
            d = radius * (0.4 + 0.2 * (i % 3))
            x, y = 32 + math.cos(angle) * d, 32 + math.sin(angle) * d
            alpha = int(255 * fade)
            draw.line((x - 2, y, x + 2, y), fill=(255, 255, 255, alpha))
            draw.line((x, y - 2, x, y + 2), fill=(255, 255, 255, alpha))
        sheet.paste(layer, (frame * FRAME, 0))
    return sheet


# The poison barrel: the wooden staves mapped colour by colour to a dark, toxic green, like the
# bog under it; the iron bands stay iron. Lightest to darkest wood.
POISON_WOOD = {
    (153, 100, 62): (70, 128, 54),
    (139, 89, 54): (54, 106, 46),
    (125, 80, 49): (42, 88, 40),
    (122, 72, 50): (34, 74, 36),
    (118, 68, 49): (34, 74, 36),
    (117, 68, 48): (34, 74, 36),
    (114, 65, 48): (30, 66, 34),
    (113, 64, 47): (30, 66, 34),
    (110, 62, 46): (30, 66, 34),
    (108, 59, 45): (26, 58, 32),
    (104, 56, 44): (22, 50, 30),
}
TOXIC = (176, 240, 72, 255)


def poison_wood(image):
    out = image.copy()
    pixels = out.load()
    for y in range(out.height):
        for x in range(out.width):
            r, g, b, a = pixels[x, y]
            if a and (r, g, b) in POISON_WOOD:
                pixels[x, y] = (*POISON_WOOD[(r, g, b)], a)
    return out


# A bubble of poison rising off the barrel, 5x5 pixels per frame (shown three times over):
# forming, round, round with its shine slid over, bursting.
BUBBLE = [
    [".....", ".....", "..g..", ".gLg.", "..g.."],
    [".ggg.", "gLllg", "glllg", "glllg", ".ggg."],
    [".ggg.", "glLlg", "glllg", "glllg", ".ggg."],
    ["g...g", ".....", ".....", ".....", "g...g"],
]
BUBBLE_PAL = {"g": (120, 210, 60, 255), "l": (70, 150, 50, 150), "L": (230, 255, 190, 255)}


def bubble_sheet():
    sheet = Image.new("RGBA", (5 * len(BUBBLE), 5), (0, 0, 0, 0))
    for index, rows in enumerate(BUBBLE):
        for y, line in enumerate(rows):
            for x, key in enumerate(line):
                if key in BUBBLE_PAL:
                    sheet.putpixel((index * 5 + x, y), BUBBLE_PAL[key])
    return sheet


# The poison puddle in the style of the floor's liquids: 32x32 masks shown four times over, one
# mask pixel to one liquid pixel. The inside is left half transparent, and the game fills it with
# churning poison (quad.wgsl, `poison_color`); the rim is drawn: a dark outline, a lit inner edge
# along the top and a few toxic glints. Row 0: the puddle spreading out of the broken barrel in
# six frames; row 1: it lying there (the liquid moves on its own, the rim glints shift).
PUDDLE_FRAME = 32
PUDDLE_BLOBS = [(16, 17, 10, 7), (9, 18, 5, 4), (23, 16, 5, 4), (14, 12, 5, 3), (18, 22, 6, 3)]
PUDDLE_GROW = [0.25, 0.45, 0.65, 0.8, 0.92, 1.0]
PUDDLE_INSIDE = (40, 80, 36, 140)
PUDDLE_OUTLINE = (12, 28, 16, 255)
PUDDLE_EDGE = (58, 104, 42, 255)
PUDDLE_GLINTS = [[(10, 13), (22, 12)], [(12, 12), (24, 13)], [(11, 13), (21, 11)]]


def puddle_shape(scale):
    inside = set()
    for y in range(PUDDLE_FRAME):
        for x in range(PUDDLE_FRAME):
            for cx, cy, rx, ry in PUDDLE_BLOBS:
                ex = 16 + (cx - 16) * scale
                ey = 17 + (cy - 17) * scale
                if ((x + 0.5 - ex) / max(1.0, rx * scale)) ** 2 + ((y + 0.5 - ey) / max(1.0, ry * scale)) ** 2 <= 1.0:
                    inside.add((x, y))
                    break
    return inside


def puddle_mask(scale, glints):
    image = Image.new("RGBA", (PUDDLE_FRAME, PUDDLE_FRAME), (0, 0, 0, 0))
    inside = puddle_shape(scale)
    for x, y in inside:
        image.putpixel((x, y), PUDDLE_INSIDE)
    for x, y in inside:
        if (x, y - 1) not in inside:
            image.putpixel((x, y), PUDDLE_EDGE)
    for x, y in inside:
        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            if (x + dx, y + dy) not in inside and 0 <= x + dx < PUDDLE_FRAME and 0 <= y + dy < PUDDLE_FRAME:
                image.putpixel((x + dx, y + dy), PUDDLE_OUTLINE)
    for x, y in glints:
        if (x, y) in inside:
            image.putpixel((x, y), TOXIC)
    return image


def liquid_puddle():
    sheet = Image.new("RGBA", (PUDDLE_FRAME * 6, PUDDLE_FRAME * 2), (0, 0, 0, 0))
    for frame, scale in enumerate(PUDDLE_GROW):
        sheet.paste(puddle_mask(scale, []), (frame * PUDDLE_FRAME, 0))
    for frame in range(6):
        sheet.paste(puddle_mask(1.0, PUDDLE_GLINTS[frame % 3]), (frame * PUDDLE_FRAME, PUDDLE_FRAME))
    return sheet


def main():
    barrel = Image.open(SRC).convert("RGBA")
    poison = poison_marks(poison_wood(barrel))
    bubble_sheet().save(f"{OUT}/poison_bubble.tga")
    frost = frost_speckles(recolor(barrel, 0.56, (0.7, 0.0), 1.25, 0.1))
    poison.save(f"{OUT}/poison_barrel.tga")
    frost.save(f"{OUT}/frost_barrel.tga")
    liquid_puddle().save(f"{OUT}/poison_puddle.tga")
    frost_burst().save(f"{OUT}/frost_burst.tga")


main()
