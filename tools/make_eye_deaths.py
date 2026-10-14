import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
BAYER = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]]


def lerp(a, b, t):
    return tuple(int(round(a[i] + (b[i] - a[i]) * t)) for i in range(3))


def squash(frame, factor, widen=1.0):
    bbox = frame.getbbox()
    if not bbox:
        return frame.copy()
    left, top, right, bottom = bbox
    body = frame.crop(bbox)
    new_h = max(1, round(body.height * factor))
    new_w = max(1, round(body.width * widen))
    body = body.resize((new_w, new_h), Image.NEAREST)
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    cx = (left + right) // 2
    out.paste(body, (cx - new_w // 2, bottom - new_h), body)
    return out


def recolor(frame, tint, amount, bright=0.0):
    out = frame.copy()
    for y in range(frame.height):
        for x in range(frame.width):
            p = frame.getpixel((x, y))
            if p[3] == 0:
                continue
            c = lerp(p, tint, amount)
            if bright:
                c = tuple(min(255, int(v + 255 * bright)) for v in c)
            out.putpixel((x, y), c + (p[3],))
    return out


def dissolve(frame, level, from_top=True):
    out = frame.copy()
    for y in range(frame.height):
        for x in range(frame.width):
            if out.getpixel((x, y))[3] == 0:
                continue
            threshold = BAYER[y % 4][x % 4] / 16.0
            fade = level * (1.0 + (0.5 - y / frame.height) * (0.7 if from_top else -0.7))
            if threshold < fade:
                out.putpixel((x, y), (0, 0, 0, 0))
    return out


def is_light(p):
    return p[3] and p[0] > 200 and p[1] > 190 and p[2] > 190


def close_eye(frame, colour):
    out = frame.copy()
    for y in range(frame.height):
        for x in range(frame.width):
            if is_light(out.getpixel((x, y))):
                out.putpixel((x, y), colour + (255,))
    return out


def demon_eye():
    sheet = Image.open(OUT / "demoneye.tga").convert("RGBA").crop((0, 0, 304, 64))
    frame_w = 76
    base = sheet.crop((0, 0, frame_w, 64))
    dark = (96, 22, 30)
    frames = [
        recolor(base, (255, 255, 255), 0.0, 0.12),
        recolor(squash(base, 0.86, 1.03), dark, 0.12),
        close_eye(recolor(squash(base, 0.66, 1.08), dark, 0.25), (104, 26, 36)),
        close_eye(recolor(squash(base, 0.44, 1.14), dark, 0.4), (84, 20, 30)),
        dissolve(close_eye(recolor(squash(base, 0.28, 1.2), dark, 0.55), (70, 18, 26)), 0.55),
    ]
    out = Image.new("RGBA", (304 + 0, 128), (0, 0, 0, 0))
    out.paste(sheet, (0, 0))
    for i, frame in enumerate(frames):
        out.paste(frame, (i * frame_w, 64))
    # a fifth frame needs a 5th column: widen the sheet
    wide = Image.new("RGBA", (frame_w * 5, 128), (0, 0, 0, 0))
    wide.paste(out, (0, 0))
    for i, frame in enumerate(frames):
        wide.paste(frame, (i * frame_w, 64))
    return wide


def dissolve_bottom(frame, level):
    out = frame.copy()
    for y in range(frame.height):
        r = y / frame.height
        chance = max(0.0, min(1.0, level * (0.55 + 0.6 * r)))
        for x in range(frame.width):
            if out.getpixel((x, y))[3] and BAYER[y % 4][x % 4] / 16.0 < chance:
                out.putpixel((x, y), (0, 0, 0, 0))
    return out


def is_eye(p):
    return p[3] and p[0] > 200 and 90 < p[1] < 200 and p[2] < 90


def dim_eye(frame, amount):
    out = frame.copy()
    for y in range(frame.height):
        for x in range(frame.width):
            p = out.getpixel((x, y))
            if is_eye(p):
                out.putpixel((x, y), lerp(p, (96, 44, 30), amount) + (255,))
    return out


def only_eye(frame, drop):
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    for y in range(frame.height):
        for x in range(frame.width):
            p = frame.getpixel((x, y))
            if is_eye(p) and 0 <= y + drop < frame.height:
                out.putpixel((x, y + drop), p)
    return out


def shift_down(frame, dy):
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    out.paste(frame, (0, dy))
    return out


def eye_watcher():
    sheet = Image.open(OUT / "eye_watcher.tga").convert("RGBA").crop((0, 0, 160, 64))
    base = sheet.crop((0, 0, 32, 32))
    frames = [
        recolor(base, (255, 255, 255), 0.0, 0.18),
        dissolve_bottom(shift_down(dim_eye(base, 0.15), 1), 0.18),
        dissolve_bottom(shift_down(dim_eye(base, 0.4), 2), 0.5),
        dim_eye(only_eye(dissolve_bottom(shift_down(base, 4), 0.9), 0), 0.6),
        dissolve_bottom(dim_eye(only_eye(shift_down(base, 7), 0), 0.8), 0.7),
    ]
    out = Image.new("RGBA", (160, 96), (0, 0, 0, 0))
    out.paste(sheet, (0, 0))
    for i, frame in enumerate(frames):
        out.paste(frame, (i * 32, 64))
    return out


def main():
    demon_eye().save(OUT / "demoneye.tga")
    eye_watcher().save(OUT / "eye_watcher.tga")


main()
