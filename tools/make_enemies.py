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
CLOUD_FRAMES = 6
PUFF = [(88, 132, 66, 150), (122, 168, 84, 140), (168, 210, 112, 150), (214, 240, 150, 165)]


def shifted(mask, dx, dy):
    out = Image.new("L", mask.size, 0)
    out.paste(mask, (dx, dy))
    return out


def puff_mask(row, index):
    mask = Image.new("L", (CLOUD, CLOUD), 0)
    d = ImageDraw.Draw(mask)
    grow = min(1.0, 0.25 + 0.15 * (index + 1)) if row == 0 else 1.0
    base = [(24, 24, 11), (14, 19, 8), (24, 12, 8), (35, 18, 8), (36, 30, 8), (24, 36, 8), (13, 31, 8), (6, 8, 3), (42, 10, 2), (41, 41, 3), (6, 41, 2)]
    for n, (cx, cy, radius) in enumerate(base):
        bob = math.sin(index * 1.05 + n * 1.7) * (1.8 if row == 1 else 0.0)
        bob_y = math.cos(index * 0.9 + n * 2.1) * (1.6 if row == 1 else 0.0)
        px = 24 + (cx - 24) * grow + bob
        py = 24 + (cy - 24) * grow + bob_y
        rad = max(2.0, radius * grow)
        d.ellipse((px - rad, py - rad, px + rad, py + rad), fill=255)
    return mask


def cloud_frame(row, index):
    mask = puff_mask(row, index)
    wide = mask.filter(ImageFilter.MaxFilter(3))
    outline = ImageChops.subtract(wide, mask)
    light = ImageChops.subtract(mask, shifted(mask, 3, 3))
    dark = ImageChops.subtract(mask, shifted(mask, -3, -3))
    image = Image.new("RGBA", (CLOUD, CLOUD), (0, 0, 0, 0))
    image.paste(Image.new("RGBA", image.size, PUFF[1]), mask=mask)
    image.paste(Image.new("RGBA", image.size, PUFF[0]), mask=dark.point(lambda v: 255 if v > 0 else 0))
    image.paste(Image.new("RGBA", image.size, PUFF[2]), mask=light.point(lambda v: 255 if v > 0 else 0))
    image.paste(Image.new("RGBA", image.size, PUFF[3]), mask=ImageChops.subtract(light, shifted(light, 2, 2)).point(lambda v: 255 if v > 0 else 0))
    image.paste(Image.new("RGBA", image.size, (58, 98, 48, 170)), mask=outline.point(lambda v: 255 if v > 0 else 0))
    return image


def cloud_sheet():
    sheet = Image.new("RGBA", (CLOUD * CLOUD_FRAMES, CLOUD * 2), (0, 0, 0, 0))
    for row in range(2):
        for index in range(CLOUD_FRAMES):
            sheet.paste(cloud_frame(row, index), (index * CLOUD, row * CLOUD))
    return sheet


def main():
    bloated_sheet().save(OUT / "bloated_spider.tga")
    cloud_sheet().save(OUT / "toxic_cloud.tga")


main()
