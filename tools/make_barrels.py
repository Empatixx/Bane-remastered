import colorsys
from PIL import Image, ImageDraw

SRC = "assets/textures/ingame/barrel.tga"
OUT = "assets/textures/ingame"
FRAME = 64
METAL_SATURATION = 0.3


def recolor(image, hue, saturation, value_scale, value_add):
    out = image.copy()
    pixels = out.load()
    for y in range(out.height):
        for x in range(out.width):
            r, g, b, a = pixels[x, y]
            if a == 0:
                continue
            h, s, v = colorsys.rgb_to_hsv(r / 255, g / 255, b / 255)
            if s < METAL_SATURATION:
                continue
            s = min(1.0, s * saturation[0] + saturation[1])
            v = min(1.0, v * value_scale + value_add)
            r, g, b = colorsys.hsv_to_rgb(hue, s, v)
            pixels[x, y] = (int(r * 255), int(g * 255), int(b * 255), a)
    return out


def frost_speckles(image):
    pixels = image.load()
    for row in range(image.height // FRAME):
        for column in range(image.width // FRAME):
            for y in range(FRAME // 2):
                for x in range(FRAME):
                    px, py = column * FRAME + x, row * FRAME + y
                    r, g, b, a = pixels[px, py]
                    if a == 0:
                        continue
                    if ((x * 73856093) ^ (y * 19349663) ^ (column * 83492791)) % 19 == 0:
                        pixels[px, py] = (235, 248, 255, a)
    return image


def poison_marks(image):
    pixels = image.load()
    for row in range(image.height // FRAME):
        for column in range(image.width // FRAME):
            for y in range(FRAME // 3, FRAME):
                for x in range(FRAME):
                    px, py = column * FRAME + x, row * FRAME + y
                    r, g, b, a = pixels[px, py]
                    if a == 0:
                        continue
                    if ((x * 73856093) ^ (y * 19349663) ^ (column * 83492791)) % 31 == 0:
                        pixels[px, py] = (190, 255, 90, a)
    return image


def puddle():
    size, frames = 64, 4
    sheet = Image.new("RGBA", (size * frames, size), (0, 0, 0, 0))
    blobs = [(32, 35, 26, 19), (14, 40, 12, 9), (50, 38, 12, 9), (29, 22, 15, 8), (37, 51, 15, 8), (44, 24, 9, 6)]
    bubbles = [
        [(22, 33, 3), (40, 40, 2), (31, 45, 2), (47, 31, 1)],
        [(22, 31, 3), (41, 38, 3), (30, 44, 1), (46, 30, 2)],
        [(23, 30, 2), (40, 36, 3), (32, 43, 3), (45, 29, 3)],
        [(24, 32, 1), (39, 39, 2), (31, 46, 3), (47, 33, 2)],
    ]
    drops = [
        [(8, 22), (54, 20), (58, 46)],
        [(7, 24), (55, 18)],
        [(10, 20), (56, 22), (6, 46)],
        [(9, 24), (57, 48)],
    ]
    rim, body, deep, shine = (22, 84, 34, 255), (46, 150, 56, 235), (88, 205, 78, 225), (150, 242, 120, 210)
    for frame in range(frames):
        layer = Image.new("RGBA", (size, size), (0, 0, 0, 0))
        draw = ImageDraw.Draw(layer)
        for cx, cy, rx, ry in blobs:
            draw.ellipse((cx - rx, cy - ry, cx + rx, cy + ry), fill=rim)
        for cx, cy, rx, ry in blobs:
            draw.ellipse((cx - rx + 2, cy - ry + 2, cx + rx - 2, cy + ry - 2), fill=body)
        for cx, cy, rx, ry in blobs[:4]:
            draw.ellipse((cx - rx + 5, cy - ry + 4, cx + rx - 5, cy + ry - 4), fill=deep)
        draw.ellipse((24, 24, 40, 32), fill=shine)
        draw.ellipse((12, 36, 20, 40), fill=shine)
        for bx, by, radius in bubbles[frame]:
            draw.ellipse((bx - radius, by - radius, bx + radius, by + radius), fill=(175, 255, 150, 235), outline=(40, 120, 50, 255))
            draw.point((bx - 1, by - 1), fill=(255, 255, 255, 255))
        for dx, dy in drops[frame]:
            draw.rectangle((dx, dy, dx + 1, dy + 1), fill=body)
        sheet.paste(layer, (frame * size, 0))
    return sheet


def main():
    barrel = Image.open(SRC).convert("RGBA")
    poison = poison_marks(recolor(barrel, 0.27, (1.0, 0.12), 0.95, 0.0))
    frost = frost_speckles(recolor(barrel, 0.56, (0.7, 0.0), 1.25, 0.1))
    poison.save(f"{OUT}/poison_barrel.tga")
    frost.save(f"{OUT}/frost_barrel.tga")
    puddle().save(f"{OUT}/poison_puddle.tga")
    preview = Image.new("RGBA", (barrel.width * 3 + 20, barrel.height + 90), (60, 60, 60, 255))
    for index, image in enumerate([barrel, poison, frost]):
        preview.paste(image, (index * (barrel.width + 10), 0), image)
    p = puddle()
    preview.paste(p, (0, barrel.height + 10), p)
    big = p.resize((p.width * 4, p.height * 4), Image.NEAREST)
    big.save('/private/tmp/claude-501/-Users-jirka-IdeaProjects-redissun/232009fd-0059-46c5-af18-fa12d7ec52a4/scratchpad/puddle_new.png')
    preview = preview.resize((preview.width * 3, preview.height * 3), Image.NEAREST)
    preview.save("/private/tmp/claude-501/-Users-jirka-IdeaProjects-redissun/232009fd-0059-46c5-af18-fa12d7ec52a4/scratchpad/barrels.png")


main()
