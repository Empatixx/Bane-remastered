import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/guns"

CLEAR = (0, 0, 0, 0)
OUTLINE = (0, 0, 0, 255)
STEEL_D = (48, 50, 54, 255)
STEEL = (86, 88, 93, 255)
STEEL_L = (149, 150, 155, 255)
SHINE = (248, 248, 248, 255)
WOOD_D = (76, 24, 20, 255)
WOOD = (133, 74, 67, 255)
WOOD_L = (181, 123, 115, 255)
WOOD_S = (243, 197, 177, 255)
OLIVE_D = (52, 62, 34, 255)
OLIVE = (88, 104, 56, 255)
OLIVE_L = (138, 156, 92, 255)
BRASS_D = (138, 98, 34, 255)
BRASS = (204, 160, 62, 255)
BRASS_L = (244, 214, 128, 255)
LENS = (92, 170, 214, 255)
HULL_L = (226, 230, 236, 255)
HULL = (176, 184, 196, 255)
HULL_D = (108, 116, 132, 255)
LASER_D = (150, 16, 24, 255)
LASER = (240, 40, 48, 255)
LASER_L = (255, 170, 170, 255)
PLASMA_D = (24, 120, 40, 255)
PLASMA = (70, 230, 90, 255)
PLASMA_L = (200, 255, 190, 255)
VOLT_D = (30, 70, 190, 255)
VOLT = (80, 160, 255, 255)
VOLT_L = (210, 236, 255, 255)
COPPER_D = (120, 56, 24, 255)
COPPER = (196, 110, 52, 255)
COPPER_L = (240, 170, 110, 255)
RAIL_D = (20, 110, 140, 255)
RAIL = (60, 220, 240, 255)
RAIL_L = (200, 252, 255, 255)


PIXEL = 2


def downsample(img, k):
    w, h = img.size
    out = Image.new("RGBA", ((w + k - 1) // k, (h + k - 1) // k), CLEAR)
    for y in range(out.height):
        for x in range(out.width):
            block = [img.getpixel((x * k + i, y * k + j)) for j in range(k) for i in range(k)
                     if x * k + i < w and y * k + j < h]
            solid = [p for p in block if p[3]]
            if len(solid) * 2 >= len(block):
                out.putpixel((x, y), max(set(solid), key=solid.count))
    return out


class Canvas:
    def __init__(self, w, h):
        self.img = Image.new("RGBA", (w, h), CLEAR)
        self.d = ImageDraw.Draw(self.img)

    def rect(self, x0, y0, x1, y1, c):
        self.d.rectangle((x0, y0, x1, y1), fill=c)

    def px(self, x, y, c):
        self.img.putpixel((x, y), c)

    def poly(self, pts, c):
        self.d.polygon(pts, fill=c)

    def line(self, pts, c):
        self.d.line(pts, fill=c)

    def bar(self, x0, y0, x1, y1, light, mid, dark):
        self.rect(x0, y0, x1, y1, mid)
        self.rect(x0, y0, x1, y0, light)
        if y1 > y0 + 1:
            self.rect(x0, y1, x1, y1, dark)

    def steel(self, x0, y0, x1, y1):
        self.bar(x0, y0, x1, y1, STEEL_L, STEEL, STEEL_D)

    def wood(self, x0, y0, x1, y1):
        self.bar(x0, y0, x1, y1, WOOD_L, WOOD, WOOD_D)

    def hull(self, x0, y0, x1, y1):
        self.bar(x0, y0, x1, y1, HULL_L, HULL, HULL_D)

    def glow(self, x0, y0, x1, y1, dark, mid, light):
        self.rect(x0, y0, x1, y1, dark)
        if y1 - y0 >= 3:
            self.rect(x0, y0 + 1, x1, y1 - 1, mid)
        if y1 - y0 >= 5:
            self.rect(x0 + 2, y0 + 2, x1 - 2, y1 - 3, light)

    def outlined(self):
        self.img = downsample(self.img, PIXEL)
        src = self.img.copy()
        w, h = src.size
        for y in range(h):
            for x in range(w):
                if src.getpixel((x, y))[3]:
                    continue
                for dy in (-1, 0, 1):
                    for dx in (-1, 0, 1):
                        nx, ny = x + dx, y + dy
                        if 0 <= nx < w and 0 <= ny < h:
                            p = src.getpixel((nx, ny))
                            if p[3] and p != OUTLINE:
                                self.img.putpixel((x, y), OUTLINE)
                                break
                    else:
                        continue
                    break
        return self.img


def sniper():
    c = Canvas(128, 34)
    # stock
    c.poly([(3, 15), (34, 17), (34, 23), (26, 23), (19, 29), (3, 29)], WOOD)
    c.line([(3, 15), (34, 17)], WOOD_L)
    c.line([(5, 17), (24, 18)], WOOD_S)
    c.line([(3, 29), (19, 29), (26, 23), (34, 23)], WOOD_D)
    c.rect(18, 22, 20, 24, WOOD_D)
    c.rect(1, 14, 2, 30, STEEL_D)
    # receiver
    c.steel(34, 15, 62, 22)
    c.rect(44, 17, 52, 17, SHINE)
    c.line([(54, 16), (54, 21)], OUTLINE)
    # bolt handle
    c.rect(50, 23, 51, 24, STEEL)
    c.rect(47, 24, 50, 26, STEEL_L)
    c.px(47, 26, STEEL_D)
    # trigger + guard
    c.line([(38, 23), (38, 27), (46, 27), (46, 23)], STEEL_D)
    c.line([(42, 23), (41, 25)], STEEL_L)
    # forend
    c.poly([(62, 16), (90, 17), (90, 21), (62, 22)], WOOD)
    c.line([(62, 16), (90, 17)], WOOD_L)
    c.line([(66, 18), (84, 18)], WOOD_S)
    c.line([(62, 22), (90, 21)], WOOD_D)
    # folded bipod
    c.rect(72, 23, 94, 23, STEEL_D)
    c.rect(72, 22, 73, 23, STEEL)
    # barrel
    c.steel(90, 17, 118, 19)
    c.rect(96, 17, 112, 17, STEEL_L)
    # muzzle brake
    c.steel(118, 15, 125, 21)
    c.line([(121, 15), (121, 21)], OUTLINE)
    c.px(119, 16, SHINE)
    # scope
    c.steel(32, 5, 40, 12)
    c.steel(40, 7, 62, 10)
    c.rect(44, 7, 58, 7, SHINE)
    c.steel(62, 4, 70, 13)
    c.line([(66, 4), (66, 13)], STEEL_D)
    c.rect(70, 6, 71, 11, LENS)
    c.px(70, 6, SHINE)
    c.rect(48, 4, 50, 6, STEEL)
    c.rect(48, 4, 50, 4, STEEL_L)
    # scope mounts
    c.rect(43, 11, 45, 14, STEEL_D)
    c.rect(56, 11, 58, 14, STEEL_D)
    return c.outlined()


def rpg():
    c = Canvas(122, 36)
    # rear flare
    c.poly([(2, 10), (12, 13), (12, 21), (2, 24)], STEEL)
    c.line([(2, 10), (12, 13)], STEEL_L)
    c.line([(2, 24), (12, 21)], STEEL_D)
    # tube
    c.bar(12, 13, 88, 21, STEEL_L, STEEL, STEEL_D)
    c.rect(18, 15, 38, 15, SHINE)
    # wooden heat shields
    c.wood(36, 12, 56, 22)
    c.line([(40, 14), (52, 14)], WOOD_S)
    for x in (44, 48):
        c.line([(x, 15), (x, 21)], WOOD_D)
    # sight
    c.rect(58, 6, 62, 12, STEEL_D)
    c.rect(59, 7, 61, 7, STEEL_L)
    c.rect(52, 9, 57, 10, STEEL_D)
    # grips
    c.poly([(40, 23), (45, 23), (44, 32), (38, 32)], WOOD)
    c.line([(40, 23), (38, 32)], WOOD_L)
    c.poly([(60, 22), (65, 22), (64, 30), (58, 30)], STEEL_D)
    c.line([(61, 22), (59, 29)], STEEL)
    c.line([(46, 23), (46, 26), (52, 26), (52, 22)], STEEL_D)
    # warhead
    c.bar(88, 14, 94, 20, OLIVE_L, OLIVE, OLIVE_D)
    c.poly([(94, 14), (100, 7), (106, 7), (118, 15), (118, 19), (106, 27), (100, 27), (94, 20)], OLIVE)
    c.line([(94, 14), (100, 7), (106, 7), (118, 15)], OLIVE_L)
    c.line([(94, 20), (100, 27), (106, 27), (118, 19)], OLIVE_D)
    c.line([(99, 10), (104, 10)], OLIVE_L)
    c.line([(102, 8), (102, 26)], OLIVE_D)
    c.rect(118, 16, 120, 18, BRASS)
    c.px(119, 16, BRASS_L)
    return c.outlined()


def minigun():
    c = Canvas(104, 46)
    # carry handle
    c.line([(16, 6), (16, 2), (36, 2), (36, 6)], STEEL_D)
    c.line([(17, 2), (35, 2)], STEEL)
    # rear grip
    c.poly([(4, 10), (10, 10), (10, 22), (4, 22)], STEEL_D)
    c.line([(5, 11), (5, 21)], STEEL)
    # motor housing
    c.bar(10, 6, 42, 26, STEEL_L, STEEL, STEEL_D)
    c.rect(14, 8, 30, 8, SHINE)
    for x in (18, 24, 30):
        c.line([(x, 11), (x, 22)], STEEL_D)
    c.rect(34, 12, 39, 18, STEEL_D)
    c.rect(35, 13, 38, 13, (220, 60, 50, 255))
    # barrels
    for y in (9, 14, 19):
        c.bar(42, y, 98, y + 3, STEEL_L, STEEL, STEEL_D)
    c.line([(42, 13), (98, 13)], OUTLINE)
    c.line([(42, 18), (98, 18)], OUTLINE)
    c.rect(60, 9, 80, 9, SHINE)
    # clamps
    for x0 in (52, 76, 94):
        c.steel(x0, 7, x0 + 3, 24)
        c.line([(x0 + 1, 7), (x0 + 1, 23)], STEEL_L)
    # ammo belt
    for i, y in enumerate(range(27, 44, 4)):
        x = 20 + i
        c.rect(x, y, x + 9, y + 2, BRASS)
        c.rect(x, y, x + 9, y, BRASS_L)
        c.rect(x + 7, y, x + 9, y + 2, BRASS_D)
    return c.outlined()


def crossbow():
    c = Canvas(88, 42)
    # stock
    c.poly([(3, 17), (40, 18), (40, 23), (30, 23), (22, 30), (3, 30)], WOOD)
    c.line([(3, 17), (40, 18)], WOOD_L)
    c.line([(6, 19), (26, 20)], WOOD_S)
    c.line([(3, 30), (22, 30), (30, 23), (40, 23)], WOOD_D)
    c.rect(1, 16, 2, 31, STEEL_D)
    # tiller
    c.wood(40, 18, 72, 22)
    c.rect(38, 23, 44, 25, STEEL_D)
    c.steel(72, 16, 76, 24)
    # prod
    c.poly([(72, 17), (67, 2), (71, 1), (77, 17)], STEEL)
    c.poly([(72, 23), (67, 38), (71, 39), (77, 23)], STEEL_D)
    c.line([(69, 2), (73, 15)], STEEL_L)
    # string
    c.line([(68, 3), (46, 18)], WOOD_S)
    c.line([(68, 37), (46, 22)], WOOD_S)
    # bolt
    c.rect(46, 16, 80, 17, WOOD_L)
    c.poly([(80, 13), (86, 16), (80, 19)], STEEL_L)
    c.rect(46, 14, 51, 16, (200, 50, 46, 255))
    return c.outlined()


def flamethrower():
    c = Canvas(98, 44)
    # stock + rear grip
    c.poly([(3, 12), (24, 13), (24, 19), (3, 22)], STEEL_D)
    c.poly([(20, 19), (26, 19), (24, 31), (18, 31)], WOOD)
    c.line([(20, 19), (18, 31)], WOOD_L)
    # body
    c.steel(24, 10, 60, 19)
    c.rect(28, 12, 50, 13, SHINE)
    # fuel tank
    c.bar(30, 21, 58, 32, (230, 104, 88, 255), (176, 44, 38, 255), (98, 20, 20, 255))
    c.rect(34, 23, 52, 24, (250, 170, 150, 255))
    c.rect(56, 24, 61, 28, STEEL_D)
    # barrel + heat shroud
    c.steel(60, 12, 86, 17)
    for x in range(64, 84, 6):
        c.rect(x, 12, x + 1, 17, STEEL_D)
    c.steel(86, 9, 92, 20)
    c.rect(93, 12, 95, 17, (90, 160, 255, 255))
    # front grip
    c.poly([(66, 18), (72, 18), (71, 29), (64, 29)], STEEL_D)
    return c.outlined()


def blaster():
    c = Canvas(56, 38)
    # grip
    c.poly([(12, 18), (22, 18), (19, 34), (8, 34)], STEEL_D)
    c.poly([(13, 19), (16, 19), (12, 33), (10, 33)], STEEL)
    c.glow(12, 22, 16, 30, LASER_D, LASER, LASER_L)
    # trigger guard
    c.rect(22, 18, 31, 19, STEEL_D)
    c.rect(29, 18, 31, 24, STEEL_D)
    c.rect(24, 18, 25, 22, STEEL_L)
    # body
    c.hull(6, 8, 40, 18)
    c.poly([(6, 8), (12, 4), (30, 4), (34, 8)], HULL_L)
    c.rect(14, 6, 28, 7, SHINE)
    c.glow(14, 11, 34, 15, LASER_D, LASER, LASER_L)
    # emitter
    c.steel(40, 9, 46, 17)
    c.rect(46, 11, 50, 15, STEEL_D)
    c.rect(50, 11, 53, 15, LASER)
    c.rect(51, 12, 53, 14, LASER_L)
    return c.outlined()


def plasma_rifle():
    c = Canvas(104, 46)
    # stock
    c.poly([(2, 14), (26, 14), (26, 24), (14, 24), (6, 32), (2, 32)], HULL_D)
    c.poly([(2, 14), (26, 14), (26, 17), (2, 17)], HULL)
    c.rect(6, 20, 14, 26, CLEAR)
    # grip
    c.poly([(30, 26), (38, 26), (36, 40), (27, 40)], STEEL_D)
    c.rect(30, 27, 32, 38, STEEL)
    # body
    c.hull(24, 10, 68, 26)
    c.rect(28, 12, 50, 13, SHINE)
    c.rect(40, 26, 47, 30, STEEL_D)
    # plasma chamber
    c.steel(46, 6, 66, 20)
    c.glow(48, 8, 64, 18, PLASMA_D, PLASMA, PLASMA_L)
    for x in (52, 58):
        c.rect(x, 8, x + 1, 18, STEEL_D)
    # barrel and prongs
    c.steel(68, 13, 88, 21)
    c.glow(70, 15, 86, 19, PLASMA_D, PLASMA, PLASMA)
    c.poly([(86, 8), (98, 12), (98, 14), (86, 13)], HULL)
    c.poly([(86, 21), (98, 22), (98, 24), (86, 26)], HULL_D)
    c.rect(94, 15, 98, 19, PLASMA_L)
    return c.outlined()


def tesla_gun():
    c = Canvas(96, 50)
    # grip and battery
    c.poly([(16, 26), (25, 26), (22, 42), (12, 42)], WOOD)
    c.poly([(16, 26), (19, 26), (15, 41), (12, 41)], WOOD_L)
    c.steel(26, 28, 40, 38)
    c.glow(28, 30, 38, 36, VOLT_D, VOLT, VOLT_L)
    # body
    c.steel(8, 14, 50, 27)
    c.rect(12, 16, 40, 17, STEEL_L)
    c.rect(4, 16, 8, 25, STEEL_D)
    # coils
    c.steel(48, 16, 76, 24)
    for x in range(49, 74, 6):
        c.bar(x, 12, x + 3, 28, COPPER_L, COPPER, COPPER_D)
    # emitter fork and orb
    c.rect(74, 12, 78, 28, STEEL_D)
    c.poly([(78, 12), (84, 8), (86, 10), (80, 15)], STEEL)
    c.poly([(78, 28), (84, 32), (86, 30), (80, 25)], STEEL)
    c.d.ellipse((80, 13, 93, 27), fill=VOLT_D)
    c.d.ellipse((82, 15, 91, 25), fill=VOLT)
    c.d.ellipse((84, 16, 88, 21), fill=VOLT_L)
    return c.outlined()


def railgun():
    c = Canvas(132, 40)
    # stock
    c.poly([(2, 14), (30, 15), (30, 24), (20, 24), (12, 32), (2, 32)], STEEL_D)
    c.poly([(2, 14), (30, 15), (30, 18), (2, 18)], STEEL)
    # grip
    c.poly([(36, 24), (44, 24), (42, 36), (33, 36)], STEEL_D)
    c.rect(36, 25, 38, 34, STEEL)
    # capacitor body
    c.hull(28, 8, 64, 24)
    c.rect(32, 10, 56, 11, SHINE)
    for x in (40, 47, 54):
        c.glow(x, 14, x + 4, 21, RAIL_D, RAIL, RAIL_L)
    # rails
    c.steel(62, 8, 124, 13)
    c.steel(62, 20, 124, 25)
    c.glow(62, 14, 120, 19, RAIL_D, RAIL, RAIL_L)
    for x in (76, 94, 112):
        c.rect(x, 6, x + 3, 27, HULL_D)
        c.rect(x, 6, x + 1, 27, HULL)
    c.rect(124, 12, 128, 21, RAIL_L)
    # scope
    c.steel(36, 2, 56, 7)
    c.rect(56, 3, 58, 6, RAIL)
    return c.outlined()


GUNS = {
    "sniper": sniper,
    "rpg": rpg,
    "minigun": minigun,
    "crossbow": crossbow,
    "flamethrower": flamethrower,
    "blaster": blaster,
    "plasma_rifle": plasma_rifle,
    "tesla_gun": tesla_gun,
    "railgun": railgun,
}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for name, draw in GUNS.items():
        img = draw()
        img.save(OUT / f"{name}.tga")
        print(name, img.size)


if __name__ == "__main__":
    main()
