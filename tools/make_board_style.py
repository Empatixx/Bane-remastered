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


GLYPHS = {
    "damage": [
        ".........",
        "....X....",
        "...XXX...",
        "...XXX...",
        "...XXX...",
        "...XXX...",
        "..XXXXX..",
        "..XXXXX..",
        ".........",
    ],
    "rate": [
        ".........",
        ".....XX..",
        "....XX...",
        "...XX....",
        "..XXXXX..",
        "....XX...",
        "...XX....",
        "..XX.....",
        ".........",
    ],
    "magazine": [
        ".........",
        ".X..X..X.",
        "XXXXXXXXX",
        "XX.XX.XX.",
        "XX.XX.XX.",
        "XX.XX.XX.",
        "XX.XX.XX.",
        "XXXXXXXXX",
        ".........",
    ],
}


CREAM = (232, 214, 170)
DEEP = (46, 30, 24)


def glyph(rows, ink, outline):
    size = 11
    image = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    filled = set()
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key == "X":
                filled.add((x + 1, y + 1))
    for x, y in filled:
        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            if (x + dx, y + dy) not in filled:
                image.putpixel((x + dx, y + dy), outline + (255,))
    for x, y in filled:
        image.putpixel((x, y), ink + (255,))
    return image


def upgrade_icons():
    names = ["damage", "rate", "magazine"]
    sheet = Image.new("RGBA", (11, 11 * 3), (0, 0, 0, 0))
    for i, name in enumerate(names):
        sheet.paste(glyph(GLYPHS[name], TALENT_ORANGE, LINE), (0, i * 11))
    return sheet


def upgrade_node():
    sheet = Image.new("RGBA", (16, 64), (0, 0, 0, 0))
    for i, state in enumerate(["owned", "available", "locked", "chosen"]):
        sheet.paste(node(state), (0, i * 16))
    return sheet


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
    style_board(60, 159).save(f"{OUT}/weapon_list.tga")
    style_board(130, 159).save(f"{OUT}/weapon_tree.tga")
    preview([board, notes, talent_icon(), upgrade_icons(), style_board(60, 159), style_board(130, 159)]).save(f"{SCRATCH}/board_a.png")


main()
