import os
import random
import sys

from PIL import Image, ImageDraw

SCRATCH = "/private/tmp/claude-501/-Users-jirka-IdeaProjects-redissun/232009fd-0059-46c5-af18-fa12d7ec52a4/scratchpad"
OUT = sys.argv[1] if len(sys.argv) > 1 else f"{SCRATCH}/board_new"

LINE = (37, 19, 26)
GOLD = (196, 154, 86)
GOLD_DARK = (161, 118, 48)
GOLD_MID = (189, 140, 62)
GOLD_LIGHT = (230, 177, 92)
BROWN = (97, 67, 38)
BROWN_LIGHT = (122, 83, 51)
BROWN_HI = (143, 102, 67)
BROWN_DARK = (77, 53, 31)
BROWN_DEEP = (58, 40, 23)
FOOT = (49, 49, 66)
FOOT_LIGHT = (71, 71, 94)
PAPER = (189, 171, 147)
PAPER_LIGHT = (214, 198, 172)
PAPER_SHADE = (171, 143, 106)
PAPER_EDGE = (112, 102, 88)
BLUE = (181, 181, 235)
BLUE_MID = (113, 113, 199)
BLUE_LIGHT = (216, 216, 232)
BLUE_LINE = (165, 165, 212)
BLUE_DARK = (96, 96, 125)
YELLOW = (255, 232, 110)
PIN = (176, 58, 44)
PIN_LIGHT = (240, 130, 110)
SHADOW = (20, 10, 14, 120)


def box(draw, x0, y0, x1, y1, fill, line=LINE):
    draw.rectangle((x0, y0, x1, y1), fill=line + (255,))
    draw.rectangle((x0 + 1, y0 + 1, x1 - 1, y1 - 1), fill=fill + (255,))


def planks(draw, x0, y0, x1, y1, base, seam, light, grain, plank, seed):
    r = random.Random(seed)
    for y in range(y0, y1 + 1, plank):
        bottom = min(y + plank - 1, y1)
        draw.rectangle((x0, y, x1, bottom), fill=base + (255,))
        draw.line((x0, y, x1, y), fill=light + (255,))
        draw.line((x0, bottom, x1, bottom), fill=seam + (255,))
        for _ in range(4):
            gx = r.randint(x0 + 2, max(x0 + 3, x1 - 24))
            gy = r.randint(y + 3, max(y + 3, bottom - 3))
            draw.line((gx, gy, gx + r.randint(6, 22), gy), fill=grain + (255,))
        sx = r.randint(x0 + 10, x1 - 10)
        draw.line((sx, y, sx, bottom), fill=seam + (255,))


def notice_board(w, h):
    image = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    for x in (8, w - 21):
        box(draw, x, h - 14, x + 12, h - 1, FOOT)
        draw.line((x + 1, h - 13, x + 11, h - 13), fill=FOOT_LIGHT + (255,))
    for x in (6, w - 17):
        box(draw, x, 14, x + 10, h - 12, BROWN)
        draw.line((x + 1, 15, x + 1, h - 13), fill=BROWN_LIGHT + (255,))
    box(draw, 14, 14, w - 15, h - 21, BROWN)
    planks(draw, 15, 17, w - 16, h - 22, BROWN, BROWN_DARK, BROWN_LIGHT, BROWN_DARK, 15, 3)
    draw.rectangle((15, 15, w - 16, 18), fill=BROWN_DEEP + (255,))
    draw.line((15, 19, w - 16, 19), fill=BROWN_DARK + (255,))
    draw.line((15, 15, 15, h - 22), fill=BROWN_DEEP + (255,))
    draw.line((w - 16, 15, w - 16, h - 22), fill=BROWN_DEEP + (255,))
    box(draw, 3, h - 22, w - 4, h - 13, BROWN_LIGHT)
    draw.line((4, h - 21, w - 5, h - 21), fill=BROWN_HI + (255,))
    draw.line((4, h - 14, w - 5, h - 14), fill=BROWN_DARK + (255,))
    box(draw, 0, 0, w - 1, 17, GOLD)
    for y, color in ((3, GOLD_LIGHT), (9, GOLD_MID), (14, GOLD_DARK), (15, GOLD_DARK)):
        draw.line((1, y, w - 2, y), fill=color + (255,))
    r = random.Random(9)
    for _ in range(30):
        x = r.randint(3, w - 24)
        y = r.choice([5, 6, 11, 12])
        draw.line((x, y, x + r.randint(5, 20), y), fill=r.choice([GOLD_LIGHT, GOLD_MID]) + (255,))
    for x in (w // 3, 2 * w // 3 + 9):
        draw.line((x, 1, x, 16), fill=GOLD_DARK + (255,))
    return image


def table(w, h):
    image = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    box(draw, 0, 0, w - 1, h - 1, GOLD)
    planks(draw, 1, 1, w - 2, h - 17, GOLD, GOLD_DARK, GOLD_LIGHT, GOLD_MID, 17, 5)
    draw.line((1, 1, w - 2, 1), fill=GOLD_LIGHT + (255,))
    draw.line((1, 1, 1, h - 17), fill=GOLD_LIGHT + (255,))
    draw.line((w - 2, 1, w - 2, h - 17), fill=GOLD_DARK + (255,))
    box(draw, 0, h - 16, w - 1, h - 1, BROWN_DARK)
    draw.line((1, h - 15, w - 2, h - 15), fill=BROWN_LIGHT + (255,))
    draw.line((1, h - 3, w - 2, h - 3), fill=BROWN_DEEP + (255,))
    for x in (12, w - 13):
        draw.rectangle((x - 1, 5, x + 1, 7), fill=BROWN_DARK + (255,))
        draw.rectangle((x - 1, h - 22, x + 1, h - 20), fill=BROWN_DARK + (255,))
    return image


def pin(draw, cx, cy):
    draw.rectangle((cx - 1, cy, cx + 2, cy + 2), fill=SHADOW)
    draw.rectangle((cx - 1, cy - 2, cx + 1, cy), fill=PIN + (255,))
    draw.point((cx - 1, cy - 2), fill=PIN_LIGHT + (255,))
    draw.rectangle((cx - 1, cy - 3, cx + 1, cy - 3), fill=LINE + (255,))


def paper(w, h, tint, lift, stamp, seed):
    image = Image.new("RGBA", (w + 4, h + 4), (0, 0, 0, 0))
    base = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(base)
    fill = (232, 218, 194) if tint else PAPER_LIGHT
    box(draw, 0, 0, w - 1, h - 1, fill)
    draw.line((1, 1, w - 2, 1), fill=(244, 232, 210, 255))
    draw.rectangle((1, h - 4, w - 2, h - 2), fill=PAPER_SHADE + (255,))
    r = random.Random(seed)
    for _ in range(int(w * h * 0.004)):
        draw.point((r.randint(2, w - 3), r.randint(2, h - 3)), fill=PAPER_SHADE + (255,))
    draw.polygon([(w - 8, h - 1), (w - 1, h - 1), (w - 1, h - 8)], fill=(0, 0, 0, 0))
    draw.polygon([(w - 8, h - 1), (w - 8, h - 8), (w - 1, h - 8)], fill=PAPER_EDGE + (255,))
    draw.line((w - 8, h - 8, w - 1, h - 8), fill=LINE + (255,))
    draw.line((w - 8, h - 8, w - 8, h - 1), fill=LINE + (255,))
    pin(draw, w // 2, 5)
    if stamp:
        green = (56, 140, 76, 255)
        draw.ellipse((w - 21, 7, w - 6, 22), outline=green)
        draw.line((w - 17, 15, w - 14, 18), fill=green)
        draw.line((w - 14, 18, w - 9, 11), fill=green)
    shadow = Image.new("RGBA", image.size, (0, 0, 0, 0))
    shadow.paste(SHADOW, (3, 3), base.split()[3])
    image.alpha_composite(shadow)
    image.paste(base, (1, 1), base)
    if lift:
        outline = Image.new("RGBA", image.size, (0, 0, 0, 0))
        mask = Image.new("L", image.size, 0)
        mask.paste(base.split()[3], (1, 1))
        for dx in (-1, 0, 1):
            for dy in (-1, 0, 1):
                if dx or dy:
                    outline.paste(YELLOW + (255,), (dx, dy), mask)
        outline.alpha_composite(image)
        image = outline
    return image


def note_paper():
    w, h = 40, 46
    rows = [paper(w, h, False, False, False, 1), paper(w, h, True, True, False, 1), paper(w, h, False, False, True, 2), paper(w, h, True, True, True, 2)]
    sheet = Image.new("RGBA", (w + 4, (h + 4) * len(rows)), (0, 0, 0, 0))
    for i, row in enumerate(rows):
        sheet.paste(row, (0, i * (h + 4)))
    return sheet


def tag(w, h, state):
    image = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    fill = {"normal": GOLD_MID, "hover": GOLD, "chosen": GOLD_LIGHT}[state]
    box(draw, 0, 0, w - 1, h - 1, fill)
    draw.line((1, 1, w - 2, 1), fill=GOLD_LIGHT + (255,))
    draw.line((1, h - 2, w - 2, h - 2), fill=GOLD_DARK + (255,))
    for y in (3, h - 4):
        draw.point((3, y), fill=BROWN_DEEP + (255,))
    if state == "chosen":
        draw.rectangle((w - 5, 2, w - 3, h - 3), fill=BROWN_DEEP + (255,))
        draw.rectangle((w - 4, 3, w - 4, h - 4), fill=YELLOW + (255,))
    if state == "hover":
        draw.rectangle((0, 0, w - 1, h - 1), outline=YELLOW + (255,))
    return image


def note_tag():
    w, h = 52, 14
    sheet = Image.new("RGBA", (w, h * 3), (0, 0, 0, 0))
    for i, state in enumerate(["normal", "hover", "chosen"]):
        sheet.paste(tag(w, h, state), (0, i * h))
    return sheet


def blueprint(w, h):
    image = Image.new("RGBA", (w + 4, h + 4), (0, 0, 0, 0))
    base = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(base)
    box(draw, 0, 0, w - 1, h - 1, BLUE, FOOT_LIGHT)
    draw.rectangle((1, 1, w - 2, h - 2), outline=BLUE_MID + (255,))
    for x in range(14, w - 3, 8):
        draw.line((x, 2, x, h - 3), fill=BLUE_LINE + (255,))
    for y in range(8, h - 3, 8):
        draw.line((9, y, w - 3, y), fill=BLUE_LINE + (255,))
    draw.line((12, 21, w - 6, 21), fill=BLUE_MID + (255,))
    box(draw, 0, -2, 8, h + 1, BLUE_LIGHT, FOOT_LIGHT)
    draw.line((2, 1, 2, h - 2), fill=(255, 255, 255, 255))
    draw.line((6, 1, 6, h - 2), fill=BLUE_DARK + (255,))
    shadow = Image.new("RGBA", image.size, (0, 0, 0, 0))
    shadow.paste(SHADOW, (3, 3), base.split()[3])
    image.alpha_composite(shadow)
    image.paste(base, (1, 1), base)
    return image


def plaque(w, h, state):
    image = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    rim = {"normal": BROWN_HI, "hover": YELLOW, "disabled": BROWN_DARK}[state]
    fill = {"normal": BROWN, "hover": BROWN_LIGHT, "disabled": BROWN_DEEP}[state]
    box(draw, 0, 0, w - 1, h - 1, rim)
    draw.rectangle((2, 2, w - 3, h - 3), fill=fill + (255,))
    for cx, cy in ((3, 3), (w - 4, 3), (3, h - 4), (w - 4, h - 4)):
        draw.point((cx, cy), fill=LINE + (255,))
    return image


def note_plaque():
    w, h = 22, 7
    sheet = Image.new("RGBA", (w, h * 3), (0, 0, 0, 0))
    for i, state in enumerate(["normal", "hover", "disabled"]):
        sheet.paste(plaque(w, h, state), (0, i * h))
    return sheet


ICON = [
    "....O....",
    "...OLO...",
    "...OGO...",
    "OOOOGOOOO",
    "OGGLLGGDO",
    ".OGGLGDO.",
    "..OGGGO..",
    ".OGGOGDO.",
    ".OOO.OOO.",
]


def talent_icon():
    colors = {"O": LINE, "G": GOLD_LIGHT, "L": (255, 240, 190), "D": GOLD_MID}
    image = Image.new("RGBA", (9, 9), (0, 0, 0, 0))
    for y, row in enumerate(ICON):
        for x, key in enumerate(row):
            if key in colors:
                image.putpixel((x, y), colors[key] + (255,))
    return image


import math

GOLD_RAMP = ((70, 40, 15), (150, 95, 30), (214, 150, 50), (250, 210, 100))
STEEL_RAMP = ((40, 36, 60), (86, 86, 120), (130, 132, 170), (190, 192, 225))
RED_RAMP = ((60, 15, 30), (120, 35, 50), (180, 65, 70), (230, 120, 110))
TEAL_RAMP = ((15, 50, 50), (25, 120, 110), (40, 170, 150), (110, 230, 200))
PURPLE_RAMP = ((45, 15, 55), (100, 40, 110), (150, 70, 160), (210, 130, 215))
GREEN_RAMP = ((15, 50, 25), (35, 110, 50), (60, 165, 80), (140, 225, 140))
ICE_RAMP = ((30, 50, 80), (70, 120, 170), (130, 190, 225), (220, 245, 255))
ORANGE_RAMP = ((70, 25, 15), (160, 60, 20), (225, 110, 35), (255, 190, 90))
PINK_RAMP = ((60, 20, 40), (140, 70, 95), (205, 130, 150), (245, 200, 210))
DARK_RAMP = ((20, 8, 16), (30, 14, 24), (44, 20, 34), (70, 36, 52))


class Icon:
    def __init__(self, outline=None):
        self.layers = []
        self.outline = outline

    def layer(self, ramp, paint):
        mask = Image.new("L", (16, 16), 0)
        paint(ImageDraw.Draw(mask))
        self.layers.append((ramp, mask))
        return self

    def render(self):
        out = Image.new("RGBA", (18, 18), (0, 0, 0, 0))
        solid = Image.new("L", (18, 18), 0)
        for ramp, mask in self.layers:
            shape = Image.new("L", (18, 18), 0)
            shape.paste(mask, (1, 1))
            pixels = shape.load()

            def lit(x, y):
                return 0 <= x < 18 and 0 <= y < 18 and pixels[x, y] > 0

            for y in range(18):
                for x in range(18):
                    if not lit(x, y):
                        continue
                    up = not lit(x, y - 1) or not lit(x - 1, y)
                    down = not lit(x, y + 1) or not lit(x + 1, y)
                    tone = ramp[3] if up and not down else ramp[1] if down and not up else ramp[2]
                    out.putpixel((x, y), tone + (255,))
                    solid.putpixel((x, y), 255)
        line = self.outline or self.layers[0][0][0]
        edge = Image.new("RGBA", (18, 18), (0, 0, 0, 0))
        spots = solid.load()
        for y in range(18):
            for x in range(18):
                if spots[x, y]:
                    continue
                for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                    nx, ny = x + dx, y + dy
                    if 0 <= nx < 18 and 0 <= ny < 18 and spots[nx, ny]:
                        edge.putpixel((x, y), line + (255,))
                        break
        edge.alpha_composite(out)
        return edge


def icon_damage():
    return (
        Icon()
        .layer(GOLD_RAMP, lambda d: d.rectangle((5, 8, 10, 14), fill=255))
        .layer(STEEL_RAMP, lambda d: d.polygon([(5, 8), (10, 8), (9, 4), (8, 2), (7, 2), (6, 4)], fill=255))
        .render()
    )


def icon_rate():
    return (
        Icon()
        .layer(ORANGE_RAMP, lambda d: d.polygon([(10, 0), (3, 9), (7, 9), (5, 15), (13, 6), (8, 6), (11, 0)], fill=255))
        .render()
    )


def icon_magazine():
    return (
        Icon()
        .layer(STEEL_RAMP, lambda d: d.rectangle((3, 7, 12, 14), fill=255))
        .layer(GOLD_RAMP, lambda d: [d.rectangle((x, 2, x + 1, 6), fill=255) for x in (4, 7, 10)])
        .layer(DARK_RAMP, lambda d: [d.rectangle((x, 9, x, 12), fill=255) for x in (5, 8, 11)])
        .render()
    )


def icon_reload():
    def arc(d):
        d.arc((2, 3, 13, 14), start=60, end=345, fill=255, width=3)

    return (
        Icon()
        .layer(TEAL_RAMP, arc)
        .layer(TEAL_RAMP, lambda d: d.polygon([(8, 0), (14, 4), (8, 8)], fill=255))
        .render()
    )


def icon_accuracy():
    def paint(d):
        d.ellipse((3, 3, 12, 12), outline=255, width=2)
        for box in ((7, 0, 8, 4), (7, 11, 8, 15), (0, 7, 4, 8), (11, 7, 15, 8)):
            d.rectangle(box, fill=255)

    return (
        Icon()
        .layer(RED_RAMP, paint)
        .layer(PINK_RAMP, lambda d: d.rectangle((7, 7, 8, 8), fill=255))
        .render()
    )


def icon_crit():
    return (
        Icon()
        .layer(PURPLE_RAMP, lambda d: d.polygon([(8, 0), (10, 6), (16, 8), (10, 10), (8, 16), (6, 10), (0, 8), (6, 6)], fill=255))
        .layer(PINK_RAMP, lambda d: d.polygon([(8, 4), (9, 7), (12, 8), (9, 9), (8, 12), (7, 9), (4, 8), (7, 7)], fill=255))
        .render()
    )


def icon_piercing():
    return (
        Icon()
        .layer(STEEL_RAMP, lambda d: d.rectangle((7, 1, 8, 14), fill=255))
        .layer(ICE_RAMP, lambda d: (d.rectangle((0, 7, 11, 8), fill=255), d.polygon([(15, 7), (10, 3), (10, 12)], fill=255)))
        .render()
    )


def icon_explosive():
    points = []
    for i in range(16):
        radius = 8 if i % 2 == 0 else 4.5
        angle = math.pi * i / 8
        points.append((7.5 + radius * math.cos(angle), 7.5 + radius * math.sin(angle)))
    return (
        Icon()
        .layer(ORANGE_RAMP, lambda d: d.polygon(points, fill=255))
        .layer(GOLD_RAMP, lambda d: d.ellipse((4, 4, 11, 11), fill=255))
        .render()
    )


def icon_venom():
    def paint(d):
        d.polygon([(8, 0), (3, 8), (13, 8)], fill=255)
        d.ellipse((3, 5, 12, 14), fill=255)

    return Icon().layer(GREEN_RAMP, paint).render()


def icon_frost():
    def paint(d):
        for k in range(3):
            angle = math.pi * k / 3
            dx, dy = 7.2 * math.cos(angle), 7.2 * math.sin(angle)
            d.line((7.5 - dx, 7.5 - dy, 7.5 + dx, 7.5 + dy), fill=255, width=2)
        d.ellipse((5, 5, 10, 10), fill=255)

    return Icon().layer(ICE_RAMP, paint).render()


def icon_ricochet():
    return (
        Icon()
        .layer(STEEL_RAMP, lambda d: d.rectangle((0, 0, 1, 15), fill=255))
        .layer(PURPLE_RAMP, lambda d: (d.line([(3, 13), (7, 5), (10, 11), (13, 4)], fill=255, width=2), d.polygon([(15, 0), (9, 2), (14, 7)], fill=255)))
        .render()
    )


def icon_deadeye():
    return (
        Icon()
        .layer(PINK_RAMP, lambda d: d.polygon([(0, 8), (5, 3), (11, 3), (16, 8), (11, 13), (5, 13)], fill=255))
        .layer(RED_RAMP, lambda d: d.ellipse((5, 4, 11, 12), fill=255))
        .layer(DARK_RAMP, lambda d: d.ellipse((7, 6, 9, 10), fill=255))
        .render()
    )


BONE_TOP = ((70, 40, 40), (190, 165, 150), (240, 225, 205), (255, 248, 235))
BONE_LEFT = ((70, 40, 40), (160, 130, 120), (215, 192, 172), (240, 224, 205))
BONE_RIGHT = ((70, 40, 40), (120, 92, 92), (172, 148, 140), (205, 185, 170))
PIP_RAMP = ((40, 14, 28), (60, 20, 40), (90, 30, 55), (120, 50, 75))
HOT_YELLOW = (255, 232, 110)


def dice(hover):
    top = BONE_TOP if not hover else tuple(tuple(min(255, c + 12) for c in tone) for tone in BONE_TOP)
    icon = Icon(HOT_YELLOW if hover else None)
    icon.layer(top, lambda d: d.polygon([(8, 1), (14, 4), (8, 7), (2, 4)], fill=255))
    icon.layer(BONE_LEFT, lambda d: d.polygon([(2, 5), (7, 8), (7, 14), (2, 11)], fill=255))
    icon.layer(BONE_RIGHT, lambda d: d.polygon([(9, 8), (14, 5), (14, 11), (9, 14)], fill=255))
    icon.layer(PIP_RAMP, lambda d: [d.rectangle(box, fill=255) for box in ((7, 3, 8, 4), (3, 8, 4, 9), (5, 11, 6, 12), (10, 9, 11, 10), (12, 7, 13, 8), (11, 12, 12, 13))])
    return icon.render()




def reroll_dice():
    idle, hot = dice(False), dice(True)
    sheet = Image.new("RGBA", (18, 18 * 8), (0, 0, 0, 0))
    sheet.paste(idle, (0, 0))
    sheet.paste(hot, (0, 18))
    mirrored = idle.transpose(Image.FLIP_LEFT_RIGHT)
    frames = [idle.rotate(90), idle.rotate(180), idle.rotate(270), mirrored, mirrored.rotate(90), mirrored.rotate(180)]
    for i, frame in enumerate(frames):
        sheet.paste(frame, (0, 18 * (2 + i)))
    return sheet


def enemy_arrow():
    image = Image.new("RGBA", (18, 18), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    soft = (232, 96, 96, 255)
    draw.line([(6, 3), (12, 9), (6, 15)], fill=soft, width=3)
    draw.line([(7, 4), (11, 9), (7, 14)], fill=(255, 170, 160, 255), width=1)
    return image


UPGRADE_ICONS = [icon_damage, icon_rate, icon_magazine, icon_reload, icon_accuracy, icon_crit]
MUTATION_ICONS = [icon_piercing, icon_explosive, icon_venom, icon_frost, icon_ricochet, icon_deadeye]


def icon_sheet(makers):
    sheet = Image.new("RGBA", (18, 18 * len(makers)), (0, 0, 0, 0))
    for i, maker in enumerate(makers):
        sheet.paste(maker(), (0, i * 18))
    return sheet


def upgrade_icons():
    return icon_sheet(UPGRADE_ICONS)


def mutation_icons():
    return icon_sheet(MUTATION_ICONS)


PAUSE = "assets/textures/ingame/pausebg.tga"
TALENT_RIM = (225, 152, 116)
TALENT_ORANGE = (222, 112, 38)


def style_board(w, h):
    pause = Image.open(PAUSE).convert("RGBA")
    pw, ph = pause.size
    margin = 6
    out = Image.new("RGBA", (w + margin * 2, h + margin * 2), (0, 0, 0, 0))
    draw = ImageDraw.Draw(out)
    draw.rectangle((margin, margin, margin + w - 1, margin + h - 1), fill=TALENT_RIM + (255,))
    draw.rectangle((margin + 1, margin + 1, margin + w - 2, margin + h - 2), fill=(0, 0, 0, 150))
    corners = (
        ((0, 0), (0, 0)),
        ((pw - 20, 0), (w + margin * 2 - 20, 0)),
        ((0, ph - 20), (0, h + margin * 2 - 20)),
        ((pw - 20, ph - 20), (w + margin * 2 - 20, h + margin * 2 - 20)),
    )
    for (sx, sy), (dx, dy) in corners:
        crop = pause.crop((sx, sy, sx + 20, sy + 20))
        pixels = crop.load()
        for y in range(20):
            for x in range(20):
                if pixels[x, y][3] == 255:
                    out.putpixel((dx + x, dy + y), pixels[x, y])
    return out


def preview(images):
    scale = 3
    width = sum(i.width for i in images) * scale + 20 * len(images)
    height = max(i.height for i in images) * scale + 20
    canvas = Image.new("RGBA", (width, height), (60, 50, 56, 255))
    x = 10
    for i in images:
        big = i.resize((i.width * scale, i.height * scale), Image.NEAREST)
        canvas.alpha_composite(big, (x, 10))
        x += big.width + 20
    return canvas


def main():
    os.makedirs(OUT, exist_ok=True)
    board, notes = notice_board(212, 179), note_paper()
    board.save(f"{OUT}/board_notice.tga")
    notes.save(f"{OUT}/note_paper.tga")
    talent_icon().save(f"{OUT}/talent_icon.tga")
    upgrade_icons().save(f"{OUT}/upgrade_icons.tga")
    mutation_icons().save(f"{OUT}/mutation_icons.tga")
    reroll_dice().save(f"{OUT}/reroll_dice.tga")
    enemy_arrow().save(f"{OUT}/enemy_arrow.tga")
    style_board(60, 159).save(f"{OUT}/weapon_list.tga")
    style_board(130, 159).save(f"{OUT}/weapon_tree.tga")
    preview([board, notes, talent_icon(), upgrade_icons(), mutation_icons(), style_board(60, 159), style_board(130, 159)]).save(f"{SCRATCH}/board_a.png")


main()
