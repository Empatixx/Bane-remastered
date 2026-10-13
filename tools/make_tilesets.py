import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parent.parent / "assets/textures/ingame"
SOURCE = ROOT / "tileset_2.png"
TILE = 16

KEYS = [
    (57, 51, 74),
    (37, 19, 26),
    (54, 37, 59),
    (158, 142, 136),
    (61, 51, 71),
    (125, 112, 113),
    (89, 80, 89),
    (207, 194, 176),
    (70, 62, 87),
    (87, 64, 87),
    (107, 97, 107),
]

BODY = {KEYS[0], KEYS[4], KEYS[8], KEYS[2]}
LEDGE = {KEYS[3], KEYS[5], KEYS[7], KEYS[10]}

PALETTES = {
    "ice": [
        (50, 72, 108), (14, 24, 40), (40, 58, 92), (150, 190, 214), (52, 70, 102),
        (110, 150, 180), (74, 104, 140), (226, 246, 255), (66, 92, 130), (78, 100, 150), (100, 130, 165),
    ],
    "spider": [
        (40, 36, 46), (14, 10, 16), (34, 28, 40), (150, 146, 140), (44, 38, 50),
        (112, 108, 104), (78, 74, 80), (214, 210, 196), (54, 48, 62), (70, 50, 70), (96, 92, 96),
    ],
    "lava": [
        (60, 36, 36), (20, 8, 8), (52, 28, 28), (140, 100, 90), (66, 40, 38),
        (110, 72, 64), (92, 56, 50), (200, 160, 130), (80, 48, 44), (120, 52, 36), (110, 76, 66),
    ],
    "temple": [
        (86, 72, 58), (36, 24, 18), (76, 62, 50), (206, 176, 120), (90, 76, 60),
        (168, 138, 92), (130, 106, 74), (250, 230, 170), (104, 88, 68), (120, 90, 60), (150, 124, 90),
    ],
    "sewer": [
        (46, 64, 58), (16, 26, 22), (40, 54, 50), (140, 150, 120), (50, 68, 60),
        (104, 116, 92), (78, 92, 76), (196, 208, 170), (60, 82, 72), (60, 84, 60), (90, 106, 88),
    ],
    "moss": [
        (54, 56, 50), (22, 24, 16), (46, 50, 42), (150, 148, 124), (58, 60, 52),
        (116, 114, 96), (86, 88, 74), (206, 206, 176), (66, 70, 58), (70, 76, 50), (100, 102, 86),
    ],
}


def recolor(source, palette):
    table = {KEYS[index]: palette[index] + (255,) for index in range(len(KEYS))}
    out = source.copy()
    pixels = out.load()
    for y in range(out.height):
        for x in range(out.width):
            colour = source.getpixel((x, y))
            if colour[3] and colour[:3] in table:
                pixels[x, y] = table[colour[:3]]
    return out


def scatter(source, out, mask, colours, chance, seed):
    r = random.Random(seed)
    for y in range(source.height):
        for x in range(source.width):
            pixel = source.getpixel((x, y))
            if pixel[3] and pixel[:3] in mask and r.random() < chance:
                out.putpixel((x, y), r.choice(colours) + (255,))


def clumps(source, out, mask, colours, count, seed, reach=2):
    r = random.Random(seed)
    columns, rows = source.width // TILE, source.height // TILE
    for row in range(rows):
        for column in range(columns):
            for _ in range(count):
                cx = column * TILE + r.randrange(TILE)
                cy = row * TILE + r.randrange(TILE)
                for dy in range(-reach, reach + 1):
                    for dx in range(-reach, reach + 1):
                        x, y = cx + dx, cy + dy
                        if not (0 <= x < source.width and 0 <= y < source.height):
                            continue
                        pixel = source.getpixel((x, y))
                        if pixel[3] and pixel[:3] in mask and r.random() < 0.75 - 0.2 * (abs(dx) + abs(dy)):
                            out.putpixel((x, y), r.choice(colours) + (255,))


def webs(source, out, colour, seed):
    r = random.Random(seed)
    columns, rows = source.width // TILE, source.height // TILE
    for row in range(rows):
        for column in range(columns):
            if row == 0 or r.random() < 0.5:
                continue
            ox, oy = column * TILE, row * TILE
            flip_x, flip_y = r.random() < 0.5, r.random() < 0.5
            length = r.randint(5, 8)
            for step in range(length):
                for point in ((step, 0), (0, step)):
                    px = ox + (TILE - 1 - point[0] if flip_x else point[0])
                    py = oy + (TILE - 1 - point[1] if flip_y else point[1])
                    pixel = source.getpixel((px, py))
                    if pixel[3] and pixel[:3] in BODY | LEDGE:
                        out.putpixel((px, py), colour)
            for arc in (3, 5):
                for step in range(arc + 1):
                    px = ox + (TILE - 1 - step if flip_x else step)
                    py = oy + (TILE - 1 - (arc - step) if flip_y else arc - step)
                    pixel = source.getpixel((px, py))
                    if pixel[3] and pixel[:3] in BODY | LEDGE:
                        out.putpixel((px, py), colour)


def cracks(source, out, glow, core, seed):
    r = random.Random(seed)
    columns, rows = source.width // TILE, source.height // TILE
    for row in range(rows):
        for column in range(columns):
            if r.random() < 0.4:
                continue
            x = column * TILE + r.randrange(3, TILE - 3)
            y = row * TILE + r.randrange(2, 5)
            for _ in range(r.randint(5, 9)):
                if 0 <= x < source.width and 0 <= y < source.height:
                    pixel = source.getpixel((x, y))
                    if pixel[3] and pixel[:3] in BODY:
                        out.putpixel((x, y), glow)
                        if r.random() < 0.4:
                            out.putpixel((x, y), core)
                y += 1
                x += r.choice([-1, 0, 0, 1])


def drips(source, out, colour, seed):
    r = random.Random(seed)
    columns, rows = source.width // TILE, source.height // TILE
    for row in range(rows):
        for column in range(columns):
            if r.random() < 0.5:
                continue
            x = column * TILE + r.randrange(2, TILE - 2)
            y = row * TILE + r.randrange(0, 4)
            for step in range(r.randint(3, 7)):
                if y + step >= source.height:
                    break
                pixel = source.getpixel((x, y + step))
                if pixel[3] and pixel[:3] in BODY:
                    out.putpixel((x, y + step), colour)


def variant(name, source):
    out = recolor(source, PALETTES[name])
    if name == "ice":
        scatter(source, out, BODY, [(150, 190, 230)], 0.004, 1)
        scatter(source, out, LEDGE, [(255, 255, 255), (200, 235, 255)], 0.03, 2)
    elif name == "spider":
        webs(source, out, (190, 190, 150, 255), 3)
    elif name == "lava":
        cracks(source, out, (214, 84, 28, 255), (255, 190, 60, 255), 5)
        scatter(source, out, LEDGE, [(168, 70, 40), (230, 120, 50)], 0.04, 6)
    elif name == "temple":
        scatter(source, out, LEDGE, [(255, 208, 64), (232, 176, 40)], 0.07, 7)
    elif name == "sewer":
        drips(source, out, (80, 130, 96, 255), 9)
        clumps(source, out, BODY | LEDGE, [(88, 140, 70), (110, 160, 80)], 1, 10, 1)
    elif name == "moss":
        clumps(source, out, BODY | LEDGE, [(62, 110, 48), (88, 150, 58), (112, 176, 70)], 3, 11, 2)
    return out


def main():
    out_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT
    source = Image.open(SOURCE).convert("RGBA")
    names = list(PALETTES)
    results = {"grey": source}
    for name in names:
        image = variant(name, source)
        image.save(out_dir / f"tileset_{name}.png")
        results[name] = image
    scale = 3
    gap = 14
    sheet = Image.new(
        "RGBA",
        (source.width * scale, (source.height * scale + gap) * len(results)),
        (24, 20, 28, 255),
    )
    draw = ImageDraw.Draw(sheet)
    for index, (name, image) in enumerate(results.items()):
        top = index * (source.height * scale + gap)
        draw.text((4, top + 1), name, fill=(255, 255, 255, 255))
        big = image.resize((image.width * scale, image.height * scale), Image.NEAREST)
        sheet.alpha_composite(big, (0, top + gap))
    sheet.save(Path(sys.argv[2]) if len(sys.argv) > 2 else out_dir / "tilesets_preview.png")


main()
