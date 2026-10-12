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
                        pixels[px, py] = (190, 255, 90, a)
    return image


RIM, BODY, DEEP, SHINE = (22, 84, 34, 255), (46, 150, 56, 235), (88, 205, 78, 225), (150, 242, 120, 210)
BLOBS = [(32, 35, 26, 19), (14, 40, 12, 9), (50, 38, 12, 9), (29, 22, 15, 8), (37, 51, 15, 8), (44, 24, 9, 6)]


def blob(scale, wobble):
    layer = Image.new("RGBA", (FRAME, FRAME), (0, 0, 0, 0))
    draw = ImageDraw.Draw(layer)
    parts = []
    for index, (cx, cy, rx, ry) in enumerate(BLOBS):
        w = wobble * (1 if index % 2 == 0 else -1)
        parts.append((32 + (cx - 32) * scale, 35 + (cy - 35) * scale, max(2, rx * scale + w), max(2, ry * scale + w * 0.6)))
    for cx, cy, rx, ry in parts:
        draw.ellipse((cx - rx, cy - ry, cx + rx, cy + ry), fill=RIM)
    for cx, cy, rx, ry in parts:
        if rx > 4 and ry > 4:
            draw.ellipse((cx - rx + 2, cy - ry + 2, cx + rx - 2, cy + ry - 2), fill=BODY)
    for cx, cy, rx, ry in parts[:4]:
        if rx > 8 and ry > 7:
            draw.ellipse((cx - rx + 5, cy - ry + 4, cx + rx - 5, cy + ry - 4), fill=DEEP)
    if scale > 0.6:
        draw.ellipse((24, 24, 40, 32), fill=SHINE)
        draw.ellipse((12, 36, 20, 40), fill=SHINE)
    return layer, draw


def droplets(draw, frame, count, spread, rise):
    for index in range(count):
        angle = index * 2.4 + frame
        distance = spread * (0.55 + 0.45 * ((index * 7) % 5) / 4)
        x = 32 + math.cos(angle) * distance
        y = 36 + math.sin(angle) * distance * 0.7 - rise * (1 - abs(2 * ((index * 3 % 5) / 4) - 1))
        draw.rectangle((x, y, x + 1, y + 1), fill=BODY)


def bubble(draw, x, y, radius):
    draw.ellipse((x - radius, y - radius, x + radius, y + radius), fill=(175, 255, 150, 235), outline=(40, 120, 50, 255))
    draw.point((x - 1, y - 1), fill=(255, 255, 255, 255))


def puddle():
    columns, rows = 6, 2
    sheet = Image.new("RGBA", (FRAME * columns, FRAME * rows), (0, 0, 0, 0))
    grow = [0.08, 0.28, 0.5, 0.72, 0.9, 1.0]
    for frame, scale in enumerate(grow):
        layer, draw = blob(scale, 0)
        droplets(draw, frame, 8 - frame, 10 + 26 * scale, 14 - frame * 2)
        if frame == 5:
            draw.arc((6, 14, 58, 56), 200, 340, fill=SHINE)
        sheet.paste(layer, (frame * FRAME, 0))
    rising = [
        [(22, 40, 1), (40, 44, 1), (31, 47, 1)],
        [(22, 36, 2), (40, 41, 2), (31, 44, 1), (47, 33, 1)],
        [(23, 32, 3), (40, 37, 3), (31, 42, 2), (46, 31, 2)],
        [(24, 29, 3), (39, 33, 3), (32, 39, 3), (45, 29, 3)],
        [(24, 26, 2), (38, 30, 2), (33, 36, 3), (44, 28, 3)],
        [(24, 24, 1), (37, 28, 1), (33, 33, 2), (43, 26, 2)],
    ]
    wobbles = [0, 1, 1, 0, -1, -1]
    for frame in range(columns):
        layer, draw = blob(1.0, wobbles[frame])
        for bx, by, radius in rising[frame]:
            bubble(draw, bx, by, radius)
        if frame in (2, 5):
            draw.arc((10, 18, 54, 54), 20, 160, fill=SHINE)
        if frame in (3, 4):
            droplets(draw, frame, 3, 30, 6)
        sheet.paste(layer, (frame * FRAME, FRAME))
    return sheet


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


def main():
    barrel = Image.open(SRC).convert("RGBA")
    poison = poison_marks(recolor(barrel, 0.27, (1.0, 0.12), 0.95, 0.0))
    frost = frost_speckles(recolor(barrel, 0.56, (0.7, 0.0), 1.25, 0.1))
    poison.save(f"{OUT}/poison_barrel.tga")
    frost.save(f"{OUT}/frost_barrel.tga")
    puddle().save(f"{OUT}/poison_puddle.tga")
    frost_burst().save(f"{OUT}/frost_burst.tga")
    scratch = "/private/tmp/claude-501/-Users-jirka-IdeaProjects-redissun/232009fd-0059-46c5-af18-fa12d7ec52a4/scratchpad"
    bg = (70, 70, 70, 255)
    for name, sheet in (("puddle_sheet", puddle()), ("frost_sheet", frost_burst())):
        big = sheet.resize((sheet.width * 3, sheet.height * 3), Image.NEAREST)
        canvas = Image.new("RGBA", big.size, bg)
        canvas.paste(big, (0, 0), big)
        canvas.save(f"{scratch}/{name}.png")


main()
