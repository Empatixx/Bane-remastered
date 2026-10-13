import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageFilter

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
FW, FH = 32, 20
G0 = (30, 52, 28, 255)
G1 = (52, 84, 40, 255)
G2 = (88, 128, 54, 255)
G3 = (140, 180, 78, 255)
G4 = (206, 236, 120, 255)
OUTLINE = (22, 38, 22, 255)


def lum(p):
    return (p[0] * 3 + p[1] * 6 + p[2]) / 10


def swollen(frame, swell):
    out = Image.new("RGBA", (FW, FH), (0, 0, 0, 0))
    d = ImageDraw.Draw(out)
    cx, cy = 9, 6
    rx, ry = 6 + swell, 5 + swell
    d.ellipse((cx - rx, cy - ry, cx + rx, cy + ry), fill=OUTLINE)
    d.ellipse((cx - rx + 1, cy - ry + 1, cx + rx - 1, cy + ry - 1), fill=G1)
    d.ellipse((cx - rx + 2, cy - ry + 1, cx + 1, cy), fill=G2)
    out.putpixel((cx - 3, cy - 2), G3)
    for y in range(FH):
        for x in range(FW):
            p = frame.getpixel((x, y))
            if p[3] == 0:
                continue
            r, g, b, a = p
            if r > g + 30 and r > 80 and x < 19:
                l = lum(p)
                p = (G3 if l > 120 else G2 if l > 85 else G1)
            out.putpixel((x, y), p)
    d2 = ImageDraw.Draw(out)
    for x, y in [(cx - 4, cy + 1), (cx + 3, cy - 2)]:
        d2.rectangle((x, y, x + 1, y + 1), fill=G4)
    return out


def death_frame(frame, step):
    out = swollen(frame, 2 - step)
    d = ImageDraw.Draw(out)
    r = random.Random(30 + step)
    for _ in range(4 + step * 5):
        x = 3 + r.randrange(0, 18)
        y = r.randrange(0, 12)
        out.putpixel((x, y), r.choice([G3, G4, G2]))
    if step == 2:
        for x in range(FW):
            for y in range(FH):
                if r.random() < 0.35 and out.getpixel((x, y))[3]:
                    out.putpixel((x, y), (0, 0, 0, 0))
    return out


def bloated_sheet():
    source = Image.open(OUT / "spiderling.tga").convert("RGBA")
    sheet = Image.new("RGBA", (FW * 5, FH * 2), (0, 0, 0, 0))
    for i in range(5):
        frame = source.crop((i * FW, 0, (i + 1) * FW, FH))
        sheet.paste(swollen(frame, 2), (i * FW, 0))
    for i in range(3):
        frame = source.crop((i * FW, FH, (i + 1) * FW, FH * 2))
        sheet.paste(death_frame(frame, i), (i * FW, FH))
    return sheet


CLOUD = 48
CLOUD_FRAMES = 8
PUFF = [(88, 132, 66, 150), (122, 168, 84, 140), (168, 210, 112, 150), (214, 240, 150, 165)]


def shifted(mask, dx, dy):
    out = Image.new("L", mask.size, 0)
    out.paste(mask, (dx, dy))
    return out


def solid(mask):
    return mask.point(lambda v: 255 if v > 0 else 0)


def draw_puff(image, cx, cy, radius, alpha):
    mask = Image.new("L", image.size, 0)
    d = ImageDraw.Draw(mask)
    d.ellipse((cx - radius, cy - radius, cx + radius, cy + radius), fill=255)
    lobe = max(1.5, radius * 0.6)
    for dx, dy in ((-0.7, -0.4), (0.8, -0.2), (0.1, 0.8)):
        px, py = cx + dx * radius * 0.75, cy + dy * radius * 0.75
        d.ellipse((px - lobe, py - lobe, px + lobe, py + lobe), fill=255)
    wide = mask.filter(ImageFilter.MaxFilter(3))
    outline = ImageChops.subtract(wide, mask)
    light = ImageChops.subtract(mask, shifted(mask, 2, 2))
    dark = ImageChops.subtract(mask, shifted(mask, -2, -2))
    layer = Image.new("RGBA", image.size, (0, 0, 0, 0))
    layer.paste(Image.new("RGBA", image.size, (110, 158, 74, alpha)), mask=mask)
    layer.paste(Image.new("RGBA", image.size, (74, 120, 56, alpha)), mask=solid(dark))
    layer.paste(Image.new("RGBA", image.size, (170, 210, 112, alpha)), mask=solid(light))
    layer.paste(Image.new("RGBA", image.size, (64, 104, 52, min(255, alpha + 30))), mask=solid(outline))
    image.alpha_composite(layer)


PUFFS = [(0.3, 8, 5.5), (1.4, 12, 6.5), (2.5, 15, 5), (3.6, 10, 6), (4.7, 16, 4.5), (5.6, 11, 5.5), (0.9, 5, 5), (3.1, 4, 4.5)]


def cloud_frame(index):
    image = Image.new("RGBA", (CLOUD, CLOUD), (0, 0, 0, 0))
    t = index / (CLOUD_FRAMES - 1)
    ease = 1 - (1 - t) ** 2
    alpha = int(125 * (1 - t ** 3)) + 6
    for angle, distance, radius in PUFFS:
        d = distance * (0.25 + 1.15 * ease)
        cx = 24 + math.cos(angle) * d
        cy = 24 + math.sin(angle) * d * 0.9
        draw_puff(image, cx, cy, max(1.5, radius * (0.55 + 0.55 * ease) * (1 - 0.78 * t * t)), alpha)
    return image


def cloud_sheet():
    sheet = Image.new("RGBA", (CLOUD * CLOUD_FRAMES, CLOUD), (0, 0, 0, 0))
    for index in range(CLOUD_FRAMES):
        sheet.paste(cloud_frame(index), (index * CLOUD, 0))
    return sheet


def main():
    bloated_sheet().save(OUT / "bloated_spider.tga")
    cloud_sheet().save(OUT / "toxic_cloud.tga")


main()
