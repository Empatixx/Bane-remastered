import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"


def lerp(a, b, t):
    return tuple(int(round(a[i] + (b[i] - a[i]) * t)) for i in range(3))


def darken(frame, tint, amount):
    out = frame.copy()
    for y in range(frame.height):
        for x in range(frame.width):
            p = frame.getpixel((x, y))
            if p[3]:
                out.putpixel((x, y), lerp(p, tint, amount) + (p[3],))
    return out


def shifted(frame, dx, dy):
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    out.paste(frame, (dx, dy))
    return out


def on_floor(frame, floor, dx=0):
    bbox = frame.getbbox()
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    if not bbox:
        return out
    left, top, right, bottom = bbox
    body = frame.crop(bbox)
    cx = (left + right) // 2 + dx
    out.paste(body, (cx - body.width // 2, floor - body.height), body)
    return out


def scaled(frame, fx, fy, floor):
    bbox = frame.getbbox()
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    if not bbox:
        return out
    left, top, right, bottom = bbox
    body = frame.crop(bbox)
    width = max(1, round(body.width * fx))
    height = max(1, round(body.height * fy))
    body = body.resize((width, height), Image.NEAREST)
    cx = (left + right) // 2
    out.paste(body, (cx - width // 2, floor - height), body)
    return out


def pixels_where(frame, test):
    return [(x, y) for y in range(frame.height) for x in range(frame.width) if frame.getpixel((x, y))[3] and test(frame.getpixel((x, y)))]


def centroid(points):
    return (sum(x for x, _ in points) / len(points), sum(y for _, y in points) / len(points))


def bulge(frame, points, ring, core):
    out = frame.copy()
    spots = set(points)
    for x, y in points:
        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            nx, ny = x + dx, y + dy
            if (nx, ny) not in spots and 0 <= nx < frame.width and 0 <= ny < frame.height and out.getpixel((nx, ny))[3]:
                out.putpixel((nx, ny), ring + (255,))
    for x, y in points:
        out.putpixel((x, y), lerp(out.getpixel((x, y)), core, 0.35) + (255,))
    return out


def draw_cracks(frame, centre, colour, reach):
    out = frame.copy()
    d = ImageDraw.Draw(out)
    cx, cy = int(centre[0]), int(centre[1])
    d.line([(cx - reach, cy - reach // 2), (cx - 1, cy), (cx + 1, cy + 1), (cx + reach, cy + reach // 2)], fill=colour + (255,))
    d.line([(cx, cy - reach), (cx + 1, cy - 1), (cx - 1, cy + reach)], fill=colour + (255,))
    return out


def clear(frame, points, colour):
    out = frame.copy()
    for x, y in points:
        out.putpixel((x, y), colour + (255,))
    return out


def burst(frame, centre, inner, outer, count, palette, phase=0.0, gravity=0):
    out = frame.copy()
    d = ImageDraw.Draw(out)
    cx, cy = centre
    for k in range(count):
        angle = phase + k * math.tau / count
        reach = outer * (0.75 + 0.25 * ((k * 7) % 3) / 2)
        x0, y0 = cx + math.cos(angle) * inner, cy + math.sin(angle) * inner
        x1, y1 = cx + math.cos(angle) * reach, cy + math.sin(angle) * reach + gravity * (reach / outer) ** 2
        colour = palette[k % len(palette)]
        d.line([(x0, y0), (x1, y1)], fill=colour + (255,))
        d.point((x1, y1), fill=palette[-1] + (255,))
        d.point(((x0 + x1) / 2, (y0 + y1) / 2 + 1), fill=colour + (255,))
    return out


def puddle(frame, centre, width, colours):
    out = frame.copy()
    d = ImageDraw.Draw(out)
    cx, cy = centre
    d.ellipse((cx - width, cy - 2, cx + width, cy + 2), fill=colours[0] + (255,))
    d.ellipse((cx - width + 2, cy - 1, cx + width - 3, cy + 1), fill=colours[1] + (255,))
    d.point((cx - width // 2, cy - 1), fill=colours[2] + (255,))
    return out


def is_light(p):
    return p[0] > 200 and p[1] > 190 and p[2] > 190


def is_eye_orange(p):
    return p[0] > 190 and 80 < p[1] < 200 and p[2] < 90


def is_membrane(p):
    return p[0] > 190 and p[1] > 85 and p[2] > 85 and not is_light(p)


def tear(frame, body, keep, cut):
    out = frame.copy()
    for x, y in pixels_where(frame, is_membrane):
        if math.hypot(x - body[0], y - body[1]) > keep:
            out.putpixel((x, y), (0, 0, 0, 0))
    return out


GOO = [(242, 160, 52), (252, 214, 96), (255, 244, 170)]
RED_GOO = [(150, 22, 34), (226, 70, 70), (255, 220, 220)]


def shards(frame, centre, offsets, colour=(150, 22, 34)):
    out = frame.copy()
    for dx, dy in offsets:
        x, y = int(centre[0]) + dx, int(centre[1]) + dy
        for ox in (0, 1):
            for oy in (0, 1):
                out.putpixel((x + ox, y + oy), colour + (255,))
    return out


def demon_eye():
    sheet = Image.open(OUT / "demoneye.tga").convert("RGBA").crop((0, 0, 304, 64))
    fw = 76
    down, mid, flat, up = (sheet.crop((i * fw, 0, (i + 1) * fw, 64)) for i in range(4))
    socket = (52, 8, 16)

    def frame_eye(frame):
        points = pixels_where(frame, is_light)
        return centroid(points), points

    up_eye, _ = frame_eye(up)
    f0 = bulge(shifted(up, 0, -2), [(x, y - 2) for x, y in pixels_where(up, is_light)], (255, 196, 196), (255, 255, 255))

    mid_eye, mid_points = frame_eye(mid)
    f1 = clear(tear(mid, mid_eye, 24, 0), mid_points, socket)
    f1 = shards(f1, (mid_eye[0], mid_eye[1] + 7), [(-8, 0), (-3, 4), (4, 2), (9, 5)])

    down_eye, down_points = frame_eye(down)

    flat_eye, flat_points = frame_eye(flat)
    skeleton = clear(tear(flat, flat_eye, 12, 0), flat_points, socket)
    falling = clear(tear(down, down_eye, 12, 0), down_points, socket).rotate(-14, resample=Image.NEAREST)
    f2 = shifted(falling, 0, 14)
    f3 = scaled(skeleton, 1.04, 0.6, 62)
    f4 = scaled(darken(skeleton, (60, 12, 20), 0.12), 1.08, 0.44, 62)
    frames = [f0, f1, f2, f3, f4]
    wide = Image.new("RGBA", (fw * 5, 128), (0, 0, 0, 0))
    wide.paste(sheet, (0, 0))
    for i, frame in enumerate(frames):
        wide.paste(frame, (i * fw, 64))
    return wide


def blobs(frame, centre):
    out = frame.copy()
    colours = [(240, 150, 40), (252, 200, 80), (176, 84, 30)]
    spots = [(-8, -5), (8, -6), (-9, 3), (9, 4), (0, -9), (1, 8)]
    for k, (dx, dy) in enumerate(spots):
        x, y = int(centre[0]) + dx, int(centre[1]) + dy
        for ox in range(2):
            for oy in range(2):
                if 0 <= x + ox < 32 and 0 <= y + oy < 32:
                    out.putpixel((x + ox, y + oy), colours[k % 3] + (255,))
    return out


def band(frame, low, high, slant):
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    for y in range(frame.height):
        for x in range(frame.width):
            cut = y + slant * (x - 16)
            if low <= cut < high:
                p = frame.getpixel((x, y))
                if p[3]:
                    out.putpixel((x, y), p)
    return out


def eye_watcher():
    sheet = Image.open(OUT / "eye_watcher.tga").convert("RGBA").crop((0, 0, 160, 64))
    idle = sheet.crop((0, 0, 32, 32))
    socket = (26, 12, 20)
    eye_points = pixels_where(idle, is_eye_orange)
    eye = centroid(eye_points)
    cracked = bulge(shifted(idle, 0, -1), [(x, y - 1) for x, y in eye_points], (255, 214, 96), (255, 244, 170))
    empty = clear(idle, eye_points, socket)
    f1 = blobs(shifted(empty, 0, 1), (eye[0], eye[1] + 1))

    f2 = scaled(shifted(empty, 0, 0), 0.92, 0.9, 31)
    f3 = scaled(empty, 0.78, 0.66, 31)
    f4 = scaled(empty, 0.62, 0.46, 31)
    frames = [cracked, f1, f2, f3, f4]
    out = Image.new("RGBA", (160, 96), (0, 0, 0, 0))
    out.paste(sheet, (0, 0))
    for i, frame in enumerate(frames):
        out.paste(frame, (i * 32, 64))
    return out


def main():
    demon_eye().save(OUT / "demoneye.tga")
    eye_watcher().save(OUT / "eye_watcher.tga")


main()
