from PIL import Image, ImageDraw

OUT = "assets/textures/menu"
SCRATCH = "/private/tmp/claude-501/-Users-jirka-IdeaProjects-redissun/232009fd-0059-46c5-af18-fa12d7ec52a4/scratchpad"
RIM = (225, 152, 116, 255)
HOT = (255, 232, 110, 255)
INK = (222, 112, 38, 255)
FILL = (0, 0, 0, 110)
PANEL_FILL = (0, 0, 0, 125)


def shape(w, h, inset, cut):
    l, t, r, b = inset, inset, w - 1 - inset, h - 1 - inset
    c = max(0, cut - inset)
    return [
        (l + c, t), (r - c, t), (r, t + c), (r, b - c),
        (r - c, b), (l + c, b), (l, b - c), (l, t + c),
    ]


def plate(w, h, rim, fill, cut):
    image = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    ImageDraw.Draw(image).polygon(shape(w, h, 0, cut), fill=rim)
    mask = Image.new("L", (w, h), 0)
    ImageDraw.Draw(mask).polygon(shape(w, h, 1, cut), fill=255)
    pixels, m = image.load(), mask.load()
    for y in range(h):
        for x in range(w):
            if m[x, y] > 0:
                pixels[x, y] = fill
    return image


def bracketed(inner, rim, margin):
    w, h = inner.size
    out = Image.new("RGBA", (w + margin * 2, h + margin * 2), (0, 0, 0, 0))
    out.paste(inner, (margin, margin))
    draw = ImageDraw.Draw(out)
    left, top, right, bottom = margin, margin, margin + w - 1, margin + h - 1
    gap, length = 2, 4
    for sx, sy, cx, cy in ((-1, -1, left, top), (1, -1, right, top), (-1, 1, left, bottom), (1, 1, right, bottom)):
        x0, y0 = cx + sx * gap, cy + sy * gap
        draw.line((x0, y0, x0 + (-sx) * length, y0), fill=rim)
        draw.line((x0, y0, x0, y0 + (-sy) * length), fill=rim)
    return out


def panel():
    pause = Image.open("assets/textures/ingame/pausebg.tga").convert("RGBA")
    pw, ph = pause.size
    margin = 6
    base = plate(160, 106, RIM, PANEL_FILL, 2)
    out = Image.new("RGBA", (160 + margin * 2, 106 + margin * 2), (0, 0, 0, 0))
    out.paste(base, (margin, margin))
    corners = (
        ((0, 0), (0, 0)),
        ((pw - 20, 0), (160 + margin * 2 - 20, 0)),
        ((0, ph - 20), (0, 106 + margin * 2 - 20)),
        ((pw - 20, ph - 20), (160 + margin * 2 - 20, 106 + margin * 2 - 20)),
    )
    for (sx, sy), (dx, dy) in corners:
        crop = pause.crop((sx, sy, sx + 20, sy + 20))
        pixels = crop.load()
        for y in range(20):
            for x in range(20):
                if pixels[x, y][3] == 255:
                    out.putpixel((dx + x, dy + y), pixels[x, y])
    return out


def sheet(frames):
    width = frames[0].width
    height = frames[0].height
    out = Image.new("RGBA", (width, height * len(frames)), (0, 0, 0, 0))
    for index, frame in enumerate(frames):
        out.paste(frame, (0, index * height))
    return out


def arrow(left, color):
    image = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    points = [(11, 3), (11, 12), (4, 7.5)] if left else [(4, 3), (4, 12), (11, 7.5)]
    draw.polygon(points, fill=color)
    return image


def checkbox(on, hover):
    rim = HOT if hover else RIM
    image = plate(8, 8, rim, FILL, 1)
    if on:
        ImageDraw.Draw(image).rectangle((2, 2, 5, 5), fill=HOT if hover else INK)
    return image


def handle(hover):
    image = plate(16, 16, HOT if hover else RIM, (0, 0, 0, 170), 3)
    ImageDraw.Draw(image).rectangle((5, 5, 10, 10), fill=HOT if hover else INK)
    return image


def save(name, image):
    image.save(f"{OUT}/{name}")
    return image


def main():
    parts = {
        "Menu_plate.tga": sheet([bracketed(plate(75, 19, RIM, FILL, 2), RIM, 3), bracketed(plate(75, 19, HOT, FILL, 2), HOT, 3)]),
        "Menu_plate_small.tga": sheet([bracketed(plate(42, 13, RIM, FILL, 1), RIM, 3), bracketed(plate(42, 13, HOT, FILL, 1), HOT, 3)]),
        "settings_menubar.tga": sheet([plate(50, 16, RIM, FILL, 2), plate(50, 16, HOT, FILL, 2)]),
        "settings_background.tga": panel(),
        "checkbox.tga": sheet([checkbox(False, False), checkbox(True, False), checkbox(False, True), checkbox(True, True)]),
        "slider.tga": plate(128, 8, RIM, FILL, 1),
        "slider-button.tga": sheet([handle(False), handle(True)]),
        "button_arrow.tga": sheet([arrow(True, RIM), arrow(True, HOT)]),
        "button_arrow_rotated.tga": sheet([arrow(False, RIM), arrow(False, HOT)]),
    }
    for name, image in parts.items():
        save(name, image)
    bg = (52, 40, 48, 255)
    scale = 3
    canvas = Image.new("RGBA", (1000, 760), bg)
    y = 10
    x = 10
    for name in ["Menu_plate.tga", "Menu_plate_small.tga", "settings_menubar.tga", "checkbox.tga", "slider.tga", "slider-button.tga", "button_arrow.tga", "button_arrow_rotated.tga"]:
        image = parts[name].resize((parts[name].width * scale, parts[name].height * scale), Image.NEAREST)
        if x + image.width > 990:
            x = 10
            y += 130
        canvas.paste(image, (x, y), image)
        x += image.width + 20
    panel_image = parts["settings_background.tga"]
    panel_big = panel_image.resize((panel_image.width * 3, panel_image.height * 3), Image.NEAREST)
    canvas.paste(panel_big, (10, y + 140), panel_big)
    canvas.save(f"{SCRATCH}/ui_style_preview.png")


main()
