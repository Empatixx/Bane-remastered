import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/artefacts"

SIZE = 32
CLEAR = (0, 0, 0, 0)
INK = (24, 18, 26, 255)

GOLD_D = (138, 88, 28, 255)
GOLD = (214, 160, 54, 255)
GOLD_L = (250, 222, 120, 255)
GOLD_S = (255, 250, 214, 255)
WOOD_D = (70, 40, 26, 255)
WOOD = (122, 76, 46, 255)
WOOD_L = (170, 116, 72, 255)
GLASS = (150, 186, 204, 150)
GLASS_L = (222, 240, 248, 210)
ASH_D = (78, 74, 78, 255)
ASH = (128, 122, 124, 255)
ASH_L = (176, 170, 168, 255)
BLOOD_D = (92, 8, 16, 255)
BLOOD = (160, 18, 30, 255)
BLOOD_L = (226, 64, 64, 255)
CLOAK_D = (40, 44, 70, 190)
CLOAK = (74, 82, 120, 190)
CLOAK_L = (120, 132, 176, 190)
EYE = (170, 240, 255, 255)
IRON_D = (34, 34, 40, 255)
IRON = (66, 66, 76, 255)
IRON_L = (110, 110, 122, 255)
SPIRIT_D = (40, 140, 70, 230)
SPIRIT = (90, 220, 110, 230)
SPIRIT_L = (200, 255, 190, 255)
STONE_D = (62, 64, 70, 255)
STONE = (104, 106, 112, 255)
STONE_L = (152, 154, 160, 255)
VOLT = (255, 236, 90, 255)
VOLT_L = (255, 255, 220, 255)
BONE_D = (150, 134, 104, 255)
BONE = (214, 202, 172, 255)
BONE_L = (246, 240, 222, 255)
SOCKET = (120, 20, 24, 255)


class Icon:
    def __init__(self):
        self.img = Image.new("RGBA", (SIZE, SIZE), CLEAR)
        self.d = ImageDraw.Draw(self.img)

    def rect(self, x0, y0, x1, y1, c):
        self.d.rectangle((x0, y0, x1, y1), fill=c)

    def ellipse(self, x0, y0, x1, y1, c):
        self.d.ellipse((x0, y0, x1, y1), fill=c)

    def poly(self, pts, c):
        self.d.polygon(pts, fill=c)

    def line(self, pts, c, width=1):
        self.d.line(pts, fill=c, width=width)

    def px(self, x, y, c):
        if 0 <= x < SIZE and 0 <= y < SIZE:
            self.img.putpixel((x, y), c)

    def outline(self, color=INK):
        src = self.img.copy()
        for y in range(SIZE):
            for x in range(SIZE):
                if src.getpixel((x, y))[3]:
                    continue
                near = any(
                    0 <= x + dx < SIZE and 0 <= y + dy < SIZE and src.getpixel((x + dx, y + dy))[3]
                    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))
                )
                if near:
                    self.img.putpixel((x, y), color)
        return self.img


def hourglass_of_ash():
    i = Icon()
    for y0 in (3, 26):
        i.rect(7, y0, 24, y0 + 2, WOOD)
        i.rect(7, y0, 24, y0, WOOD_L)
        i.rect(7, y0 + 2, 24, y0 + 2, WOOD_D)
    for x in (8, 23):
        i.rect(x, 6, x, 25, WOOD_D)
    i.poly([(10, 6), (21, 6), (17, 15), (17, 16), (21, 25), (10, 25), (14, 16), (14, 15)], GLASS)
    i.poly([(11, 7), (20, 7), (19, 9), (12, 9)], ASH)
    i.rect(15, 10, 16, 21, ASH_L)
    i.poly([(15, 19), (16, 19), (20, 25), (11, 25)], ASH)
    i.poly([(15, 21), (16, 21), (18, 25), (13, 25)], ASH_D)
    i.line([(11, 8), (13, 12)], GLASS_L)
    i.line([(12, 24), (13, 21)], GLASS_L)
    i.px(15, 12, ASH_D)
    i.px(16, 17, ASH_D)
    return i.outline()


def blood_chalice():
    i = Icon()
    i.poly([(5, 7), (26, 7), (24, 14), (19, 18), (12, 18), (7, 14)], GOLD)
    i.poly([(5, 7), (9, 7), (11, 16), (7, 14)], GOLD_L)
    i.poly([(21, 7), (26, 7), (24, 14), (19, 17)], GOLD_D)
    i.ellipse(5, 4, 26, 9, GOLD_D)
    i.ellipse(7, 5, 24, 8, BLOOD)
    i.ellipse(9, 5, 17, 6, BLOOD_L)
    i.rect(24, 8, 24, 12, BLOOD)
    i.px(24, 13, BLOOD_D)
    i.rect(8, 10, 8, 11, BLOOD)
    i.ellipse(14, 10, 17, 13, BLOOD_D)
    i.ellipse(14, 10, 16, 12, BLOOD)
    i.px(15, 10, BLOOD_L)
    i.rect(14, 19, 17, 22, GOLD)
    i.rect(14, 19, 14, 22, GOLD_L)
    i.rect(17, 19, 17, 22, GOLD_D)
    i.rect(13, 21, 18, 21, GOLD_D)
    i.poly([(9, 27), (22, 27), (19, 23), (12, 23)], GOLD)
    i.rect(9, 27, 22, 27, GOLD_D)
    i.rect(12, 23, 15, 23, GOLD_L)
    i.px(10, 10, GOLD_S)
    return i.outline()


def phantom_cloak():
    i = Icon()
    i.poly([(16, 2), (23, 7), (25, 16), (28, 28), (4, 28), (7, 16), (9, 7)], CLOAK)
    i.poly([(16, 2), (9, 7), (7, 16), (4, 28), (9, 28), (11, 16), (12, 9)], CLOAK_L)
    i.poly([(23, 7), (25, 16), (28, 28), (23, 28), (21, 16)], CLOAK_D)
    i.ellipse(11, 7, 21, 18, (12, 10, 22, 235))
    i.rect(13, 12, 14, 12, EYE)
    i.rect(18, 12, 19, 12, EYE)
    for x, y in ((6, 28), (10, 28), (14, 28), (18, 28), (22, 28), (26, 28)):
        i.px(x, y + 1, CLOAK)
    i.line([(13, 22), (12, 27)], CLOAK_D)
    i.line([(19, 22), (20, 27)], CLOAK_D)
    return i.outline((14, 12, 30, 255))


def grave_lantern():
    i = Icon()
    i.ellipse(12, 1, 19, 7, IRON)
    i.ellipse(14, 3, 17, 5, CLEAR)
    i.poly([(9, 9), (22, 9), (19, 6), (12, 6)], IRON)
    i.rect(12, 6, 19, 6, IRON_L)
    i.rect(9, 10, 22, 24, (30, 60, 40, 200))
    i.ellipse(12, 12, 19, 23, SPIRIT_D)
    i.poly([(15, 11), (18, 16), (19, 21), (12, 21), (13, 16)], SPIRIT)
    i.poly([(15, 14), (17, 18), (17, 21), (14, 21), (14, 18)], SPIRIT_L)
    for x in (9, 22):
        i.rect(x, 10, x, 24, IRON)
    i.rect(9, 16, 22, 16, (20, 40, 28, 120))
    i.rect(9, 10, 9, 24, IRON_L)
    i.rect(8, 24, 23, 27, IRON)
    i.rect(8, 24, 23, 24, IRON_L)
    i.rect(8, 27, 23, 27, IRON_D)
    i.px(11, 12, GLASS_L)
    i.px(11, 13, GLASS_L)
    return i.outline()


def midas_coin():
    i = Icon()
    i.ellipse(3, 3, 28, 28, GOLD_D)
    i.ellipse(3, 3, 27, 27, GOLD)
    i.ellipse(6, 6, 25, 25, GOLD_D)
    i.ellipse(6, 6, 24, 24, GOLD_L)
    i.ellipse(9, 7, 22, 20, GOLD_D)
    i.rect(11, 18, 20, 22, GOLD_D)
    i.ellipse(10, 8, 21, 19, GOLD)
    i.rect(12, 18, 19, 21, GOLD)
    i.rect(11, 12, 14, 15, BLOOD_D)
    i.rect(17, 12, 20, 15, BLOOD_D)
    i.px(15, 16, GOLD_D)
    i.px(16, 16, GOLD_D)
    for x in (13, 15, 17, 19):
        i.rect(x, 20, x, 21, GOLD_D)
    i.line([(6, 9), (9, 5)], GOLD_S)
    i.px(23, 23, GOLD_S)
    i.px(8, 8, GOLD_S)
    return i.outline()


def thunder_idol():
    i = Icon()
    i.rect(9, 25, 22, 28, STONE_D)
    i.rect(9, 25, 22, 25, STONE)
    i.poly([(11, 25), (20, 25), (21, 13), (10, 13)], STONE)
    i.poly([(11, 25), (13, 25), (12, 13), (10, 13)], STONE_L)
    i.poly([(18, 25), (20, 25), (21, 13), (19, 13)], STONE_D)
    i.ellipse(10, 5, 21, 15, STONE)
    i.ellipse(10, 5, 15, 13, STONE_L)
    i.poly([(10, 7), (8, 2), (12, 5)], STONE)
    i.poly([(21, 7), (23, 2), (19, 5)], STONE_D)
    i.rect(12, 9, 14, 10, VOLT)
    i.rect(17, 9, 19, 10, VOLT)
    i.px(13, 9, VOLT_L)
    i.px(18, 9, VOLT_L)
    i.rect(13, 18, 18, 19, STONE_D)
    i.line([(15, 13), (14, 16), (16, 17), (15, 21)], VOLT)
    bolt = [(4, 12), (7, 15), (5, 16), (8, 20)]
    i.line(bolt, VOLT)
    i.line([(27, 10), (24, 14), (26, 15), (23, 19)], VOLT)
    i.px(4, 12, VOLT_L)
    i.px(27, 10, VOLT_L)
    i.px(3, 22, VOLT)
    i.px(28, 22, VOLT)
    return i.outline()


def infinity_quiver():
    i = Icon()
    for x in (8, 14, 20):
        i.rect(x, 7, x + 4, 20, GOLD)
        i.rect(x, 7, x, 20, GOLD_L)
        i.rect(x + 4, 7, x + 4, 20, GOLD_D)
        i.poly([(x, 7), (x + 4, 7), (x + 3, 3), (x + 1, 3)], GOLD_L)
        i.px(x + 2, 2, GOLD_S)
        i.rect(x, 18, x + 4, 18, GOLD_D)
    i.rect(5, 21, 26, 28, WOOD)
    i.rect(5, 21, 26, 21, WOOD_L)
    i.rect(5, 28, 26, 28, WOOD_D)
    for x, y in ((9, 23), (10, 22), (11, 22), (12, 22), (13, 23), (14, 24), (15, 25),
                 (16, 25), (17, 26), (18, 27), (19, 27), (20, 27), (21, 26), (22, 25),
                 (21, 24), (20, 23), (19, 23), (18, 23), (17, 24), (14, 26), (13, 27),
                 (12, 27), (11, 27), (10, 27), (9, 26), (8, 25), (8, 24), (22, 24)):
        i.px(x, y, GOLD_L)
    i.px(23, 25, GOLD_L)
    i.px(22, 26, GOLD_L)
    i.px(23, 24, GOLD_L)
    i.px(10, 9, GOLD_S)
    return i.outline()


def bone_turret():
    i = Icon()
    for (x0, x1) in ((8, 4), (16, 16), (23, 27)):
        i.line([(x0, 18), (x1, 28)], BONE, width=2)
    i.line([(8, 18), (4, 28)], BONE_D)
    i.ellipse(2, 27, 6, 29, BONE_L)
    i.ellipse(14, 27, 18, 29, BONE_L)
    i.ellipse(25, 27, 29, 29, BONE_L)
    i.rect(6, 16, 25, 19, BONE_D)
    i.rect(6, 16, 25, 16, BONE)
    i.ellipse(7, 3, 22, 17, BONE)
    i.ellipse(7, 3, 14, 13, BONE_L)
    i.rect(10, 13, 19, 18, BONE)
    i.rect(10, 8, 12, 10, SOCKET)
    i.rect(16, 8, 18, 10, SOCKET)
    i.px(11, 9, BLOOD_L)
    i.px(17, 9, BLOOD_L)
    i.rect(20, 10, 29, 13, IRON)
    i.rect(20, 10, 29, 10, IRON_L)
    i.rect(20, 13, 29, 13, IRON_D)
    i.rect(28, 9, 29, 14, IRON_D)
    for x in (11, 13, 15, 17):
        i.rect(x, 15, x, 17, BONE_D)
    return i.outline()


ICONS = {
    "hourglass_of_ash": hourglass_of_ash,
    "blood_chalice": blood_chalice,
    "phantom_cloak": phantom_cloak,
    "grave_lantern": grave_lantern,
    "midas_coin": midas_coin,
    "thunder_idol": thunder_idol,
    "infinity_quiver": infinity_quiver,
    "bone_turret": bone_turret,
}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for name, draw in ICONS.items():
        draw().save(OUT / f"{name}.tga")
        print(OUT / f"{name}.tga")


if __name__ == "__main__":
    main()
