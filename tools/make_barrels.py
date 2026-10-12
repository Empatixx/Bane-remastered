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
    width, height, frames = 64, 32, 4
    sheet = Image.new("RGBA", (width * frames, height), (0, 0, 0, 0))
    blobs = [(32, 16, 28, 12), (16, 18, 14, 9), (48, 17, 14, 8), (30, 10, 16, 6), (34, 24, 18, 6)]
    bubbles = [
        [(20, 14, 3), (40, 18, 2), (30, 21, 2)],
        [(22, 12, 3), (42, 17, 3), (28, 20, 2)],
        [(24, 13, 2), (38, 15, 3), (32, 22, 3)],
        [(18, 16, 2), (36, 12, 2), (44, 20, 3)],
    ]
    for frame in range(frames):
        layer = Image.new("RGBA", (width, height), (0, 0, 0, 0))
        draw = ImageDraw.Draw(layer)
        for cx, cy, rx, ry in blobs:
            draw.ellipse((cx - rx, cy - ry, cx + rx, cy + ry), fill=(30, 110, 40, 200))
        inner = Image.new("RGBA", (width, height), (0, 0, 0, 0))
        draw = ImageDraw.Draw(inner)
        for cx, cy, rx, ry in blobs:
            draw.ellipse((cx - rx + 2, cy - ry + 2, cx + rx - 2, cy + ry - 2), fill=(70, 190, 70, 190))
        for cx, cy, rx, ry in blobs[:3]:
            draw.ellipse((cx - rx + 5, cy - ry + 4, cx + rx - 5, cy + ry - 4), fill=(120, 235, 90, 170))
        layer = Image.alpha_composite(layer, inner)
        draw = ImageDraw.Draw(layer)
        for bx, by, radius in bubbles[frame]:
            draw.ellipse((bx - radius, by - radius, bx + radius, by + radius), fill=(170, 255, 140, 230))
            draw.point((bx - 1, by - 1), fill=(255, 255, 255, 255))
        sheet.paste(layer, (frame * width, 0))
    return sheet


def main():
    barrel = Image.open(SRC).convert("RGBA")
    poison = poison_marks(recolor(barrel, 0.27, (1.0, 0.12), 0.95, 0.0))
    frost = frost_speckles(recolor(barrel, 0.56, (0.7, 0.0), 1.25, 0.1))
    poison.save(f"{OUT}/poison_barrel.tga")
    frost.save(f"{OUT}/frost_barrel.tga")
    puddle().save(f"{OUT}/poison_puddle.tga")
    preview = Image.new("RGBA", (barrel.width * 3 + 20, barrel.height + 80), (60, 60, 60, 255))
    for index, image in enumerate([barrel, poison, frost]):
        preview.paste(image, (index * (barrel.width + 10), 0), image)
    p = puddle()
    preview.paste(p, (0, barrel.height + 10), p)
    preview = preview.resize((preview.width * 3, preview.height * 3), Image.NEAREST)
    preview.save("/private/tmp/claude-501/-Users-jirka-IdeaProjects-redissun/232009fd-0059-46c5-af18-fa12d7ec52a4/scratchpad/barrels.png")


main()
