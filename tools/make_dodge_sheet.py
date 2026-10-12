import math
from PIL import Image

FW, FH = 16, 38
OUT_W = 32
SCALE = 8
CANVAS = 40

src = Image.open("assets/textures/ingame/player.tga").convert("RGBA")
side = src.crop((0, FH, FW, FH * 2))

HEAD = side.crop((0, 3, FW, 20))
TORSO = side.crop((0, 20, FW, 30))
LEGS = side.crop((0, 30, FW, 38))


def big(img):
    return img.resize((img.width * SCALE, img.height * SCALE), Image.NEAREST)


def place(canvas, part, pivot_in_part, pivot_on_canvas, cw_degrees):
    image = big(part)
    px, py = pivot_in_part[0] * SCALE, pivot_in_part[1] * SCALE
    pad = Image.new("RGBA", (CANVAS * SCALE, CANVAS * SCALE), (0, 0, 0, 0))
    pad.alpha_composite(
        image,
        (int(pivot_on_canvas[0] * SCALE - px), int(pivot_on_canvas[1] * SCALE - py)),
    )
    rotated = pad.rotate(
        -cw_degrees,
        resample=Image.NEAREST,
        center=(pivot_on_canvas[0] * SCALE, pivot_on_canvas[1] * SCALE),
    )
    canvas.alpha_composite(rotated)


def rotate_point(point, around, cw_degrees):
    angle = math.radians(cw_degrees)
    dx, dy = point[0] - around[0], point[1] - around[1]
    return (
        around[0] + dx * math.cos(angle) - dy * math.sin(angle),
        around[1] + dx * math.sin(angle) + dy * math.cos(angle),
    )


def pose(spin, torso_lean, head_bend, leg_bend):
    hip = (CANVAS / 2, CANVAS / 2 + 4)
    canvas = Image.new("RGBA", (CANVAS * SCALE, CANVAS * SCALE), (0, 0, 0, 0))
    torso_angle = spin + torso_lean
    neck = rotate_point((hip[0], hip[1] - 10), hip, torso_angle)
    leg_angle = spin + leg_bend
    place(canvas, LEGS, (8, 0), hip, leg_angle)
    place(canvas, TORSO, (8, 10), hip, torso_angle)
    place(canvas, HEAD, (8, 15), neck, torso_angle + head_bend)
    small = canvas.resize((CANVAS, CANVAS), Image.NEAREST)
    return small


def crop_to_cell(frame, ground):
    box = frame.getbbox()
    if box is None:
        return Image.new("RGBA", (OUT_W, FH), (0, 0, 0, 0))
    part = frame.crop(box)
    cell = Image.new("RGBA", (OUT_W, FH), (0, 0, 0, 0))
    x = (OUT_W - part.width) // 2
    y = FH - part.height - ground
    cell.alpha_composite(part, (x, y))
    return cell


def squash(height):
    stand = side.crop(side.getbbox())
    return stand.resize((stand.width, height), Image.NEAREST)


POSES = [
    (35, 12, 28, -70),
    (130, 18, 40, -82),
    (215, 22, 45, -85),
    (305, 14, 28, -60),
]

sheet = Image.new("RGBA", (OUT_W * 6, FH), (0, 0, 0, 0))
crouch = squash(26)
sheet.alpha_composite(crouch, ((OUT_W - crouch.width) // 2, FH - crouch.height))
for index, values in enumerate(POSES):
    frame = pose(*values)
    sheet.alpha_composite(crop_to_cell(frame, 1), ((1 + index) * OUT_W, 0))
rise = squash(31)
sheet.alpha_composite(rise, (5 * OUT_W + (OUT_W - rise.width) // 2, FH - rise.height))
sheet.save("assets/textures/ingame/player_dodge.tga", compression=None)
print(sheet.size)
