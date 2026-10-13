import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

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
PUFF = [(70, 120, 46, 110), (96, 150, 60, 130), (136, 188, 82, 150), (196, 232, 120, 175)]


def cloud_frame(row, index):
    image = Image.new("RGBA", (CLOUD, CLOUD), (0, 0, 0, 0))
    grow = (index + 1) / CLOUD_FRAMES if row == 0 else 1.0
    puffs = [(24, 24, 16), (15, 20, 10), (33, 19, 10), (17, 32, 10), (32, 31, 11), (24, 13, 8), (24, 36, 8)]
    for y in range(CLOUD):
        for x in range(CLOUD):
            density = 0.0
            for n, (cx, cy, radius) in enumerate(puffs):
                sway = math.sin(index * 1.05 + n * 1.7) * (2.2 if row == 1 else 0.0)
                sway_y = math.cos(index * 0.9 + n * 2.1) * (1.8 if row == 1 else 0.0)
                px = 24 + (cx - 24) * grow + sway
                py = 24 + (cy - 24) * grow + sway_y
                rad = max(2.0, radius * grow)
                distance = math.hypot(x - px, y - py) / rad
                if distance < 1.0:
                    density = max(density, 1.0 - distance ** 1.4)
            if density <= 0.0:
                continue
            chance = density * (0.85 if row == 0 else 0.8)
            value = noise(x, y, index + row * 40)
            if value < chance:
                level = 3 if density > 0.75 and value < 0.25 else 2 if density > 0.45 else 1 if density > 0.2 else 0
                image.putpixel((x, y), PUFF[level])
    return image


def noise(x, y, seed):
    value = (x * 374761393 + y * 668265263 + seed * 2147483647) & 0xFFFFFFFF
    value = ((value ^ (value >> 13)) * 1274126177) & 0xFFFFFFFF
    return ((value ^ (value >> 16)) & 0xFFFF) / 65535.0


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
