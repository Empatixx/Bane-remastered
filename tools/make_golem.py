import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

W = 96
OUTLINE = (30, 26, 36, 255)
STONE_D = (74, 70, 84, 255)
STONE = (116, 112, 124, 255)
STONE_L = (158, 154, 166, 255)
MOSS = (86, 112, 62, 255)
MOSS_L = (122, 150, 82, 255)
GLOW = (96, 196, 255, 255)
GLOW_HOT = (222, 246, 255, 255)
GLOW_D = (48, 120, 232, 255)
DARK = (22, 18, 28, 255)


def rock_chunk(size, seed, turn):
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    r = random.Random(seed)
    c = size / 2
    pts = []
    for k in range(8):
        a = k * math.tau / 8 + r.uniform(-0.2, 0.2)
        rad = c * r.uniform(0.62, 0.95)
        pts.append((c + math.cos(a) * rad, c + math.sin(a) * rad))
    d.polygon(pts, fill=STONE, outline=OUTLINE)
    d.polygon([(c - c * 0.6, c - c * 0.1), (c - c * 0.1, c - c * 0.65), (c + c * 0.3, c - c * 0.35)], fill=STONE_L)
    d.point((int(c + c * 0.3), int(c + c * 0.3)), fill=STONE_D)
    d.point((int(c - c * 0.3), int(c + c * 0.4)), fill=STONE_D)
    if size >= 24:
        d.point((int(c + c * 0.2), int(c - c * 0.5)), fill=MOSS)
    return im.rotate(turn, resample=Image.NEAREST)


def burst_frame(size, step, seed):
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    r = random.Random(seed + step)
    c = size / 2
    reach = size * (0.18 + 0.22 * step)
    dust = [(150, 144, 156, 255), (200, 196, 206, 255)]
    for k in range(7):
        a = k * math.tau / 7 + 0.3
        x, y = c + math.cos(a) * reach, c + math.sin(a) * reach
        w = max(1, int(size / 9 - step * 0.6))
        d.rectangle((x - w, y - w, x + w, y + w), fill=STONE if k % 2 else STONE_L, outline=OUTLINE if w > 1 else None)
    if step < 3:
        rad = size * (0.2 + 0.14 * step)
        d.ellipse((c - rad, c - rad, c + rad, c + rad), outline=dust[step % 2])
    return im


def projectile_sheet(size, seed, big):
    sheet = Image.new("RGBA", (size * 4, size * 2), (0, 0, 0, 0))
    for i in range(4):
        sheet.paste(rock_chunk(size, seed, i * 90), (i * size, 0))
        sheet.paste(burst_frame(size, i, seed), (i * size, size))
    return sheet


def rock_shadow():
    im = Image.new("RGBA", (32, 16), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    d.ellipse((1, 1, 30, 14), fill=(24, 18, 24, 150))
    return im


def spark_sheet(warm=False):
    sheet = Image.new("RGBA", (16, 4), (0, 0, 0, 0))
    colours = [GLOW_HOT, GLOW, GLOW_D, (30, 64, 150, 255)]
    if warm:
        colours = [(255, 232, 160, 255), (250, 157, 81, 255), (225, 112, 40, 255), (162, 43, 97, 255)]
    for i in range(4):
        d = ImageDraw.Draw(sheet)
        size = 4 - i
        d.rectangle((i * 4, 0, i * 4 + size - 1, size - 1), fill=colours[i])
    return sheet


def main():
    projectile_sheet(6, 21, False).save(OUT / "golem_rock.tga")
    projectile_sheet(16, 5, True).save(OUT / "golem_boulder.tga")
    rock_shadow().save(OUT / "rock_shadow.tga")
    spark_sheet().save(OUT / "spark.tga")
    spark_sheet(True).save(OUT / "spark_warm.tga")


main()
