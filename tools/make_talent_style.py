import sys

from PIL import Image, ImageDraw

SCRATCH = "/private/tmp/claude-501/-Users-jirka-IdeaProjects-redissun/232009fd-0059-46c5-af18-fa12d7ec52a4/scratchpad"
OUT = sys.argv[1] if len(sys.argv) > 1 else f"{SCRATCH}/talent_new"
PAUSE = "assets/textures/ingame/pausebg.tga"

PALETTE = {
    "common": ((225, 152, 116), (92, 62, 72), (130, 92, 92)),
    "rare": ((120, 200, 255), (44, 74, 112), (72, 112, 156)),
    "epic": ((226, 150, 255), (88, 46, 118), (136, 84, 168)),
}
DIM = {
    "common": ((110, 84, 90), (36, 26, 34), (52, 38, 48)),
    "rare": ((62, 84, 108), (26, 32, 46), (38, 46, 64)),
    "epic": ((104, 70, 120), (34, 22, 44), (50, 34, 62)),
}
ORDER = ["common", "rare", "epic"]
YELLOW = (255, 232, 90)
WHITE = (255, 255, 255)
STATES = ["owned", "locked", "available"]


def frame(quality, state):
    rim, fill, light = PALETTE[quality] if state == "owned" else DIM[quality]
    if state == "available":
        rim = YELLOW
    image = Image.new("RGBA", (32, 32), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    draw.rectangle((4, 4, 27, 27), fill=rim + (255,))
    draw.rectangle((5, 5, 26, 26), fill=(16, 10, 14, 255))
    draw.rectangle((6, 6, 25, 25), fill=fill + (255,))
    draw.rectangle((7, 7, 24, 24), fill=tuple(int(c * 0.82) for c in fill) + (255,))
    draw.rectangle((9, 9, 22, 22), fill=tuple(int(c * 0.82 + l * 0.18) for c, l in zip(fill, light)) + (255,))
    draw.rectangle((7, 7, 24, 7), fill=light + (255,))
    ornament = rim
    for sx, sy, cx, cy in ((-1, -1, 4, 4), (1, -1, 27, 4), (-1, 1, 4, 27), (1, 1, 27, 27)):
        x, y = cx + sx * 2, cy + sy * 2
        draw.line((x, y, x - sx * 4, y), fill=ornament + (255,))
        draw.line((x, y, x, y - sy * 4), fill=ornament + (255,))
    return image


def frames():
    sheet = Image.new("RGBA", (32, 32 * len(ORDER) * len(STATES)), (0, 0, 0, 0))
    for q, quality in enumerate(ORDER):
        for s, state in enumerate(STATES):
            sheet.paste(frame(quality, state), (0, (q * len(STATES) + s) * 32))
    return sheet


def root():
    image = Image.new("RGBA", (32, 32), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    rim, fill, light = PALETTE["common"]
    draw.ellipse((3, 3, 28, 28), fill=rim + (255,))
    draw.ellipse((4, 4, 27, 27), fill=(16, 10, 14, 255))
    draw.ellipse((5, 5, 26, 26), fill=fill + (255,))
    draw.ellipse((8, 8, 23, 23), fill=tuple(int(c * 0.8) for c in fill) + (255,))
    star = [(16, 6), (18, 13), (25, 16), (18, 19), (16, 26), (14, 19), (7, 16), (14, 13)]
    draw.polygon(star, fill=(255, 214, 96, 255))
    draw.polygon([(16, 10), (17, 14), (21, 16), (17, 18), (16, 22), (15, 18), (11, 16), (15, 14)], fill=(255, 244, 190, 255))
    return image


def select():
    image = Image.new("RGBA", (32, 32), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    for sx, sy, cx, cy in ((-1, -1, 4, 4), (1, -1, 27, 4), (-1, 1, 4, 27), (1, 1, 27, 27)):
        x, y = cx + sx * 3, cy + sy * 3
        draw.line((x, y, x - sx * 6, y), fill=YELLOW + (255,), width=2)
        draw.line((x, y, x, y - sy * 6), fill=YELLOW + (255,), width=2)
    return image


def board():
    pause = Image.open(PAUSE).convert("RGBA")
    pw, ph = pause.size
    rim = PALETTE["common"][0] + (255,)
    margin, w, h = 6, 200, 167
    out = Image.new("RGBA", (w + margin * 2, h + margin * 2), (0, 0, 0, 0))
    draw = ImageDraw.Draw(out)
    draw.rectangle((margin, margin, margin + w - 1, margin + h - 1), fill=rim)
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


def preview(sheet, root_image, board_image):
    canvas = Image.new("RGBA", (980, 560), (37, 19, 26, 255))
    scale = 3
    for q in range(3):
        for s in range(3):
            tile = sheet.crop((0, (q * 3 + s) * 32, 32, (q * 3 + s) * 32 + 32)).resize((32 * scale, 32 * scale), Image.NEAREST)
            canvas.alpha_composite(tile, (20 + s * 110, 20 + q * 110))
    r = root_image.resize((96, 96), Image.NEAREST)
    canvas.alpha_composite(r, (500, 20))
    b = board_image.resize((board_image.width * 2, board_image.height * 2), Image.NEAREST)
    canvas.alpha_composite(b, (640, 20))
    return canvas


def main():
    sheet, root_image, board_image = frames(), root(), board()
    sheet.save(f"{OUT}/talent_frame.tga")
    select().save(f"{OUT}/talent_select.tga")
    root_image.save(f"{OUT}/talent_root.tga")
    board_image.save(f"{OUT}/talent_board.tga")
    preview(sheet, root_image, board_image).save(f"{SCRATCH}/talent_set.png")


main()
