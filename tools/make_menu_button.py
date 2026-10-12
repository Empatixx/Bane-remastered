from PIL import Image, ImageDraw

W, H = 112, 28
OUT = "assets/textures/menu/Menu_plate.tga"
SCRATCH = "/private/tmp/claude-501/-Users-jirka-IdeaProjects-redissun/232009fd-0059-46c5-af18-fa12d7ec52a4/scratchpad"


def plate(hover):
    image = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    outline = (16, 9, 13, 255)
    if hover:
        rim, fill = (240, 168, 66, 255), (74, 40, 26, 255)
    else:
        rim, fill = (160, 98, 62, 255), (36, 22, 30, 255)
    cut = 3

    def shape(inset):
        l, t, r, b = inset, inset, W - 1 - inset, H - 1 - inset
        return [
            (l + cut, t), (r - cut, t), (r, t + cut), (r, b - cut),
            (r - cut, b), (l + cut, b), (l, b - cut), (l, t + cut),
        ]

    draw.polygon(shape(0), fill=outline)
    draw.polygon(shape(1), fill=rim)
    draw.polygon(shape(3), fill=fill)
    return image


def main():
    sheet = Image.new("RGBA", (W, H * 2), (0, 0, 0, 0))
    sheet.paste(plate(False), (0, 0))
    sheet.paste(plate(True), (0, H))
    sheet.save(OUT)
    scale = 4
    bg = (28, 18, 26, 255)
    canvas = Image.new("RGBA", (W * scale * 2 + 60, H * scale * 3 + 80), bg)
    for index in range(3):
        for state in range(2):
            frame = sheet.crop((0, state * H, W, state * H + H)).resize((W * scale, H * scale), Image.NEAREST)
            canvas.paste(frame, (20 + state * (W * scale + 20), 20 + index * (H * scale + 20)), frame)
    canvas.save(f"{SCRATCH}/menu_plate_preview.png")


main()
