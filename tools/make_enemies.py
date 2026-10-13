import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageFilter

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
FW, FH = 32, 20
G0 = (30, 52, 28, 255)
G1 = (52, 84, 40, 255)
G2 = (88, 128, 54, 255)
G3 = (140, 180, 78, 255)
G4 = (206, 236, 120, 255)
OUTLINE = (22, 38, 22, 255)


def lum(p):
    return (p[0] * 3 + p[1] * 6 + p[2]) / 10


def swollen(frame, swell):
    out = Image.new("RGBA", (FW, FH), (0, 0, 0, 0))
    d = ImageDraw.Draw(out)
    cx, cy = 9, 6
    rx, ry = 6 + swell, 5 + swell
    d.ellipse((cx - rx, cy - ry, cx + rx, cy + ry), fill=OUTLINE)
    d.ellipse((cx - rx + 1, cy - ry + 1, cx + rx - 1, cy + ry - 1), fill=G1)
    d.ellipse((cx - rx + 2, cy - ry + 1, cx + 1, cy), fill=G2)
    out.putpixel((cx - 3, cy - 2), G3)
    for y in range(FH):
        for x in range(FW):
            p = frame.getpixel((x, y))
            if p[3] == 0:
                continue
            r, g, b, a = p
            if r > g + 30 and r > 80 and x < 19:
                l = lum(p)
                p = (G3 if l > 120 else G2 if l > 85 else G1)
            out.putpixel((x, y), p)
    d2 = ImageDraw.Draw(out)
    for x, y in [(cx - 4, cy + 1), (cx + 3, cy - 2)]:
        d2.rectangle((x, y, x + 1, y + 1), fill=G4)
    return out


def death_frame(frame, step):
    out = swollen(frame, 2 - step)
    d = ImageDraw.Draw(out)
    r = random.Random(30 + step)
    for _ in range(4 + step * 5):
        x = 3 + r.randrange(0, 18)
        y = r.randrange(0, 12)
        out.putpixel((x, y), r.choice([G3, G4, G2]))
    if step == 2:
        for x in range(FW):
            for y in range(FH):
                if r.random() < 0.35 and out.getpixel((x, y))[3]:
                    out.putpixel((x, y), (0, 0, 0, 0))
    return out


def bloated_sheet():
    source = Image.open(OUT / "spiderling.tga").convert("RGBA")
    sheet = Image.new("RGBA", (FW * 5, FH * 2), (0, 0, 0, 0))
    for i in range(5):
        frame = source.crop((i * FW, 0, (i + 1) * FW, FH))
        sheet.paste(swollen(frame, 2), (i * FW, 0))
    for i in range(3):
        frame = source.crop((i * FW, FH, (i + 1) * FW, FH * 2))
        sheet.paste(death_frame(frame, i), (i * FW, FH))
    return sheet


CLOUD = 48
CLOUD_FRAMES = 8
PUFF = [(88, 132, 66, 150), (122, 168, 84, 140), (168, 210, 112, 150), (214, 240, 150, 165)]


def shifted(mask, dx, dy):
    out = Image.new("L", mask.size, 0)
    out.paste(mask, (dx, dy))
    return out


def solid(mask):
    return mask.point(lambda v: 255 if v > 0 else 0)


def draw_puff(image, cx, cy, radius, alpha):
    mask = Image.new("L", image.size, 0)
    d = ImageDraw.Draw(mask)
    d.ellipse((cx - radius, cy - radius, cx + radius, cy + radius), fill=255)
    lobe = max(1.5, radius * 0.6)
    for dx, dy in ((-0.7, -0.4), (0.8, -0.2), (0.1, 0.8)):
        px, py = cx + dx * radius * 0.75, cy + dy * radius * 0.75
        d.ellipse((px - lobe, py - lobe, px + lobe, py + lobe), fill=255)
    wide = mask.filter(ImageFilter.MaxFilter(3))
    outline = ImageChops.subtract(wide, mask)
    light = ImageChops.subtract(mask, shifted(mask, 2, 2))
    dark = ImageChops.subtract(mask, shifted(mask, -2, -2))
    layer = Image.new("RGBA", image.size, (0, 0, 0, 0))
    layer.paste(Image.new("RGBA", image.size, (110, 158, 74, alpha)), mask=mask)
    layer.paste(Image.new("RGBA", image.size, (74, 120, 56, alpha)), mask=solid(dark))
    layer.paste(Image.new("RGBA", image.size, (170, 210, 112, alpha)), mask=solid(light))
    layer.paste(Image.new("RGBA", image.size, (64, 104, 52, min(255, alpha + 30))), mask=solid(outline))
    image.alpha_composite(layer)


PUFFS = [(0.3, 8, 5.5), (1.4, 12, 6.5), (2.5, 15, 5), (3.6, 10, 6), (4.7, 16, 4.5), (5.6, 11, 5.5), (0.9, 5, 5), (3.1, 4, 4.5)]


def cloud_frame(index):
    image = Image.new("RGBA", (CLOUD, CLOUD), (0, 0, 0, 0))
    t = index / (CLOUD_FRAMES - 1)
    ease = 1 - (1 - t) ** 2
    alpha = int(125 * (1 - t ** 3)) + 6
    for angle, distance, radius in PUFFS:
        d = distance * (0.25 + 1.15 * ease)
        cx = 24 + math.cos(angle) * d
        cy = 24 + math.sin(angle) * d * 0.9
        draw_puff(image, cx, cy, max(1.5, radius * (0.55 + 0.55 * ease) * (1 - 0.78 * t * t)), alpha)
    return image


def cloud_sheet():
    sheet = Image.new("RGBA", (CLOUD * CLOUD_FRAMES, CLOUD), (0, 0, 0, 0))
    for index in range(CLOUD_FRAMES):
        sheet.paste(cloud_frame(index), (index * CLOUD, 0))
    return sheet


SW, SH = 40, 32
BLACK = (12, 12, 14, 255)
GHOST = (62, 58, 62, 255)
BODY = (30, 28, 30, 255)
BODY_L = (52, 48, 50, 255)
CAP = (196, 160, 70, 255)
CAP_L = (232, 200, 108, 255)
CAP_D = (132, 100, 44, 255)
EYE = (214, 52, 44, 255)
FANG = (240, 240, 232, 255)
GROUND = 29


def strider_leg(d, hip, foot, lift, colour):
    hx, hy = hip
    fx, fy = foot
    kx = hx + (fx - hx) * 0.55 + (4 if fx > hx else -4)
    ky = hy - lift
    d.line([(hx, hy), (kx, ky)], fill=colour)
    d.line([(kx, ky), (fx, fy)], fill=colour)
    if colour == BLACK:
        d.point((kx, ky - 1), fill=colour)


def strider(feet_front, feet_back, body_y, lift, lean=0, cap_shift=0, eyes=True):
    im = Image.new("RGBA", (SW, SH), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    cx, cy = 19 + lean, body_y
    hips = [(-3, -4), (0, -1), (4, 3), (7, 7)]
    for (hx, fx), foot_x in zip([(-3, 0), (0, 0), (4, 0), (7, 0)], feet_back):
        strider_leg(d, (cx + hx, cy + 2), (cx + foot_x, GROUND - 2), max(2, lift - 1), GHOST)
    d.ellipse((cx - 9, cy - 5, cx + 4, cy + 5), fill=BODY)
    d.ellipse((cx - 8, cy - 6 + cap_shift, cx + 3, cy + 1 + cap_shift), fill=CAP)
    d.arc((cx - 8, cy - 6 + cap_shift, cx + 3, cy + cap_shift), 190, 290, fill=CAP_L)
    d.line([(cx - 6, cy - 2 + cap_shift), (cx, cy - 2 + cap_shift)], fill=CAP_D)
    d.ellipse((cx + 3, cy - 3, cx + 11, cy + 4), fill=BODY)
    d.arc((cx + 3, cy - 3, cx + 11, cy + 3), 200, 300, fill=BODY_L)
    if eyes:
        for ex, ey in [(9, -1), (11, -1), (8, 1), (10, 1), (12, 1), (10, 3)]:
            d.point((cx + ex - 1, cy + ey), fill=EYE)
    d.line([(cx + 10, cy + 5), (cx + 10, cy + 7)], fill=FANG)
    d.line([(cx + 12, cy + 5), (cx + 12, cy + 7)], fill=FANG)
    for (hx, _), foot_x in zip([(-4, 0), (-1, 0), (3, 0), (7, 0)], feet_front):
        strider_leg(d, (cx + hx, cy + 3), (cx + foot_x, GROUND), lift, BLACK)
    return im


def stride(step):
    phase = step / 5 * math.tau
    bob = round(math.sin(phase * 2))
    front = [-13 + round(math.sin(phase + i * 1.6) * 3) for i in range(4)]
    front = [front[0], -7 + round(math.sin(phase + 1.6) * 3), 6 + round(math.sin(phase + 3.2) * 3), 14 + round(math.sin(phase + 4.8) * 3)]
    back = [-12, -6, 4, 12]
    back = [b + round(math.sin(phase + i * 1.6 + 1.2) * 3) for i, b in enumerate(back)]
    return strider(front, back, 11 + bob, 6)


def sheet_strider():
    sheet = Image.new("RGBA", (SW * 5, SH * 5), (0, 0, 0, 0))
    for i in range(5):
        sheet.paste(stride(i), (i * SW, 0))
    standing_f = [-13, -7, 6, 14]
    standing_b = [-12, -6, 4, 12]
    sheet.paste(strider([-12, -6, 5, 12], [-11, -5, 4, 11], 14, 4, lean=-1), (0, SH))
    sheet.paste(strider([-10, -5, 3, 9], [-9, -4, 3, 8], 20, 2, lean=-2, eyes=False), (SW, SH))
    final = strider([-7, -3, 2, 6], [-6, -2, 2, 5], 24, 1, lean=-3, eyes=False)
    for y in range(SH):
        for x in range(SW):
            if (x + y) % 2 == 0 and final.getpixel((x, y))[3]:
                final.putpixel((x, y), (0, 0, 0, 0))
    sheet.paste(final, (SW * 2, SH))
    sheet.paste(strider([-11, -6, 6, 13], [-10, -5, 5, 11], 17, 3), (0, SH * 2))
    sheet.paste(strider([-9, -5, 5, 10], [-8, -4, 4, 9], 21, 1, lean=1), (SW, SH * 2))
    sheet.paste(strider([-17, -10, 12, 18], [-15, -8, 10, 16], 6, 9, lean=1), (0, SH * 3))
    sheet.paste(strider([-12, -6, 6, 12], [-11, -5, 5, 11], 23, 1, lean=0), (0, SH * 4))
    sheet.paste(strider([-12, -6, 6, 12], [-11, -5, 5, 11], 16, 3, lean=0), (SW, SH * 4))
    return sheet


def leap_mark():
    image = Image.new("RGBA", (32, 16), (0, 0, 0, 0))
    d = ImageDraw.Draw(image)
    d.ellipse((1, 1, 30, 14), fill=(60, 16, 20, 110), outline=(226, 80, 56, 210))
    d.ellipse((5, 4, 26, 11), outline=(226, 80, 56, 140))
    return image


def main():
    bloated_sheet().save(OUT / "bloated_spider.tga")
    cloud_sheet().save(OUT / "toxic_cloud.tga")
    sheet_strider().save(OUT / "stilt_spider.tga")
    leap_mark().save(OUT / "leap_mark.tga")


main()
