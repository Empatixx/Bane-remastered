import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

CLEAR = (0, 0, 0, 0)
FUR_L = (132, 126, 135, 255)
FUR = (114, 109, 116, 255)
FUR_D = (104, 100, 106, 255)
FUR_DD = (88, 84, 92, 255)
TAN = (238, 195, 154, 255)
TAN_D = (217, 160, 102, 255)
EYE = (209, 58, 17, 255)
GOLD = (190, 156, 66, 255)
GOLD_D = (138, 111, 48, 255)
GOLD_L = (240, 208, 112, 255)
CAPE = (150, 38, 40, 255)
CAPE_D = (110, 26, 34, 255)
CAPE_L = (186, 58, 52, 255)
ERMINE = (236, 232, 238, 255)
ERMINE_D = (186, 182, 192, 255)
WOOD = (120, 84, 56, 255)
GEM = (226, 44, 52, 255)
NOSE = (206, 120, 128, 255)
W, H = 48, 58


def ellipse(d, cx, cy, rx, ry, fill):
    for y in range(H):
        for x in range(W):
            if ((x - cx) / rx) ** 2 + ((y - cy) / ry) ** 2 <= 1.0:
                d.point((x, y), fill=fill)


def rat_king(bob=0, hands=0, blink=False):
    image = Image.new("RGBA", (W, H), CLEAR)
    d = ImageDraw.Draw(image)
    d.line((4, 24 + bob, 4, 44), fill=WOOD)
    d.line((5, 24 + bob, 5, 44), fill=WOOD)
    d.rectangle((2, 20 + bob, 7, 24 + bob), fill=GEM)
    d.point((3, 21 + bob), fill=(255, 150, 140, 255))
    d.rectangle((3, 25 + bob, 6, 26 + bob), fill=GOLD)
    tail = [(9, 53), (5, 53), (3, 49), (2, 44), (3, 40), (6, 38)]
    for (ax, ay), (bx, by) in zip(tail, tail[1:]):
        d.line((ax, ay, bx, by), fill=TAN)
    d.point((2, 46), fill=TAN_D)
    d.point((4, 40), fill=TAN_D)
    d.polygon([(13, 25 + bob), (35, 25 + bob), (43, 54), (5, 54)], fill=CAPE)
    d.polygon([(13, 25 + bob), (20, 25 + bob), (14, 54), (5, 54)], fill=CAPE_L)
    d.polygon([(28, 25 + bob), (35, 25 + bob), (43, 54), (34, 54)], fill=CAPE_D)
    d.polygon([(19, 28 + bob), (29, 28 + bob), (31, 53), (17, 53)], fill=FUR_L)
    d.polygon([(24, 28 + bob), (29, 28 + bob), (31, 53), (24, 53)], fill=FUR)
    d.line((19, 28 + bob, 17, 53), fill=ERMINE)
    d.line((18, 28 + bob, 16, 53), fill=ERMINE)
    d.line((29, 28 + bob, 31, 53), fill=ERMINE)
    d.line((30, 28 + bob, 32, 53), fill=ERMINE_D)
    d.rectangle((5, 53, 43, 55), fill=ERMINE)
    for x in range(6, 43, 6):
        d.point((x, 54), fill=FUR_DD)
    d.rectangle((15, 25 + bob, 33, 27 + bob), fill=CAPE_D)
    d.rectangle((22, 28 + bob, 26, 31 + bob), fill=GOLD)
    d.point((23, 28 + bob), fill=GOLD_L)
    d.point((24, 30 + bob), fill=GOLD_D)
    for side, x in ((-1, 11), (1, 33)):
        d.rectangle((x, 31 + bob + hands, x + 4, 38 + bob + hands), fill=FUR_D)
        d.rectangle((x, 38 + bob + hands, x + 4, 41 + bob + hands), fill=TAN)
        d.point((x + (1 if side < 0 else 3), 41 + bob + hands), fill=TAN_D)
    for x0, x1 in ((11, 18), (30, 37)):
        d.rectangle((x0, 55, x1, 57), fill=TAN)
        d.line((x0, 57, x1, 57), fill=TAN_D)
    ellipse(d, 24, 16 + bob, 9, 8, FUR)
    ellipse(d, 22, 15 + bob, 6, 6, FUR_L)
    for cx in (14, 34):
        ellipse(d, cx, 10 + bob, 4, 4, FUR_D)
        ellipse(d, cx, 10 + bob, 2, 2, TAN)
    ellipse(d, 24, 20 + bob, 5, 4, FUR_L)
    d.rectangle((23, 19 + bob, 25, 20 + bob), fill=NOSE)
    for x in (19, 27):
        if blink:
            d.line((x, 14 + bob, x + 2, 14 + bob), fill=FUR_DD)
        else:
            d.rectangle((x, 13 + bob, x + 1, 15 + bob), fill=EYE)
    d.point((21, 23 + bob), fill=ERMINE)
    d.point((21, 24 + bob), fill=ERMINE)
    d.point((27, 23 + bob), fill=ERMINE)
    d.point((27, 24 + bob), fill=ERMINE)
    d.rectangle((15, 7 + bob, 33, 9 + bob), fill=GOLD)
    d.line((15, 7 + bob, 33, 7 + bob), fill=GOLD_L)
    d.line((15, 9 + bob, 33, 9 + bob), fill=GOLD_D)
    for x, top in ((15, 3), (19, 5), (23, 1), (28, 5), (32, 3)):
        d.polygon([(x, 7 + bob), (x + 2, 7 + bob), (x + 1, top + bob)], fill=GOLD)
        d.point((x + 1, top + 1 + bob), fill=GOLD_L)
    d.point((24, 8 + bob), fill=GEM)
    return image


def main():
    frames = [rat_king(0, 0), rat_king(1, 1), rat_king(0, 0), rat_king(1, 1, True)]
    sheet = Image.new("RGBA", (W * 4, H), CLEAR)
    for index, frame in enumerate(frames):
        sheet.paste(frame, (index * W, 0))
    sheet.save(OUT / "rat_king_preview.tga")


main()
