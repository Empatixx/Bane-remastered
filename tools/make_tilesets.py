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


def in_floor(x, y):
    return y < TILE and x // TILE < 21


def scatter(source, out, mask, colours, chance, seed):
    r = random.Random(seed)
    for y in range(source.height):
        for x in range(source.width):
            pixel = source.getpixel((x, y))
            if in_floor(x, y) and pixel[3] and pixel[:3] in mask and r.random() < chance:
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
                        if not (0 <= x < source.width and 0 <= y < source.height) or not in_floor(x, y):
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


FLOOR_COLUMNS = [5, 6] + list(range(12, 20))
DIRECTIONS = [(0, 1), (1, 0), (1, 1), (1, -1), (0, -1), (-1, 0), (-1, 1), (-1, -1)]


def crack_line(image, ox, oy, r, glow, core, mask):
    dx, dy = r.choice(DIRECTIONS)
    x, y = r.randrange(3, TILE - 3), r.randrange(3, TILE - 3)
    for _ in range(r.randint(5, 11)):
        if 0 <= x < TILE and 0 <= y < TILE:
            pixel = image.getpixel((ox + x, oy + y))
            if pixel[3] and pixel[:3] in mask:
                image.putpixel((ox + x, oy + y), core if r.random() < 0.35 else glow)
        if dx == 0:
            x += r.choice([-1, 0, 0, 1])
        elif dy == 0:
            y += r.choice([-1, 0, 0, 1])
        x += dx
        y += dy


def cracks(source, out, glow, core, seed):
    r = random.Random(seed)
    for column in range(21):
        for _ in range(r.choice([0, 0, 0, 0, 1] if column not in FLOOR_COLUMNS else [0, 0, 0, 1])):
            crack_line(out, column * TILE, 0, r, glow, core, set(map(tuple, [(60, 36, 36), (66, 40, 38), (52, 28, 28), (80, 48, 44)])))


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
                if in_floor(x, y + step) and pixel[3] and pixel[:3] in BODY:
                    out.putpixel((x, y + step), colour)


EXTRA = 8
ORIGINAL_EXTRAS = [6, 6, 6, 6, 6, 5, 14, 17]
ACCENT = {
    "grey": None,
    "ice": (190, 225, 245, 255),
    "spider": (96, 88, 112, 255),
    "lava": (214, 84, 28, 255),
    "temple": (176, 140, 80, 255),
    "sewer": (70, 112, 90, 255),
    "moss": (76, 112, 56, 255),
}


def shade(colour, factor):
    return tuple(min(255, int(channel * factor)) for channel in colour[:3]) + (255,)


def line(tile, start, end, colour):
    draw = ImageDraw.Draw(tile)
    draw.line([start, end], fill=colour)


def tone(tile, factor):
    out = tile.copy()
    for y in range(tile.height):
        for x in range(tile.width):
            pixel = tile.getpixel((x, y))
            if pixel[3]:
                out.putpixel((x, y), shade(pixel, factor) if pixel[:3] != (0, 0, 0) else pixel)
    return out


TONES = [0.94, 1.05, 0.97, 1.03, 0.92, 1.07, 0.99, 1.01]


ROTATIONS = [None, Image.ROTATE_90, Image.ROTATE_180, Image.ROTATE_270]


def extra_floor(source, name, k):
    column = FLOOR_COLUMNS[2 + k % 8]
    tile = source.crop((column * TILE, 0, (column + 1) * TILE, TILE))
    rotation = ROTATIONS[(k // 2 + k) % 4]
    if rotation is not None:
        tile = tile.transpose(rotation)
    if k % 2:
        tile = tile.transpose(Image.FLIP_LEFT_RIGHT)
    return tone(tile, TONES[k % len(TONES)])


EDGE_GROUPS = [
    ("below", [9, 10], "h"),
    ("above", [1, 2], "h"),
    ("left", [7], "v"),
    ("right", [4], "v"),
]
EDGE_VARIANTS = 8
TOTAL_COLUMNS = 21 + EXTRA + EDGE_VARIANTS * len(EDGE_GROUPS)


def mutate_edge(tile, bodies, name, flip, rng):
    out = tile.copy()
    if flip == "h" and rng.random() < 0.5:
        out = out.transpose(Image.FLIP_LEFT_RIGHT)
    if flip == "v" and rng.random() < 0.5:
        out = out.transpose(Image.FLIP_TOP_BOTTOM)
    factor = rng.choice(TONES)
    for y in range(TILE):
        for x in range(TILE):
            pixel = out.getpixel((x, y))
            if pixel[3] and pixel[:3] in bodies:
                out.putpixel((x, y), shade(pixel, factor))
    for _ in range(rng.choice([0, 1, 1, 2])):
        length = rng.randint(3, 7)
        x, y = rng.randint(3, 11), rng.randint(3, 11)
        colour = None
        for step in range(length):
            px, py = (x + step, y) if flip == "h" else (x, y + step)
            if 0 <= px < TILE and 0 <= py < TILE:
                pixel = out.getpixel((px, py))
                if pixel[3] and pixel[:3] in bodies or colour is not None and pixel[3] and shade(pixel, 1.0)[:3] != (0, 0, 0):
                    colour = colour or shade(pixel, rng.choice([0.86, 1.12]))
                    out.putpixel((px, py), colour)
    return out


def widen(image, name, clean):
    wide = Image.new("RGBA", (TOTAL_COLUMNS * TILE, image.height), (0, 0, 0, 0))
    wide.paste(image, (0, 0))
    plain = clean.crop((6 * TILE, 0, 7 * TILE, TILE))
    for k in range(EXTRA):
        wide.paste(extra_floor(image, name, k), ((21 + k) * TILE, 0))
    bodies = {PALETTES[name][index] for index in (0, 2, 4, 8)}
    column = 21 + EXTRA
    for group_index, (_, sources, flip) in enumerate(EDGE_GROUPS):
        for k in range(EDGE_VARIANTS):
            rng = random.Random(7000 + group_index * 100 + k)
            tile = image.crop((sources[k % len(sources)] * TILE, 0, (sources[k % len(sources)] + 1) * TILE, TILE))
            wide.paste(mutate_edge(tile, bodies, name, flip, rng), (column * TILE, 0))
            column += 1
    return wide


def variant(name, source):
    return recolor(source, PALETTES[name])


def main():
    out_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT
    source = Image.open(SOURCE).convert("RGBA").crop((0, 0, 21 * TILE, 2 * TILE))
    names = list(PALETTES)
    results = {"grey": source}
    for name in names:
        image = widen(variant(name, source), name, recolor(source, PALETTES[name]))
        image.save(out_dir / f"tileset_{name}.png")
        results[name] = image
    scale = 3
    gap = 14
    sheet = Image.new(
        "RGBA",
        (max(image.width for image in results.values()) * scale, (source.height * scale + gap) * len(results)),
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
