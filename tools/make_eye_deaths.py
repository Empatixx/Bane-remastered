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


def slump(frame, floor, height, width, lean, wobble, phase):
    bbox = frame.getbbox()
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    if not bbox:
        return out
    left, top, right, bottom = bbox
    cx = (left + right) / 2
    span = max(1, bottom - top)
    for y in range(top, bottom):
        t = (bottom - 1 - y) / span
        dest = floor - 1 - round((bottom - 1 - y) * height)
        sway = round(lean * t * t + math.sin(y * 0.85 + phase) * wobble * t)
        for x in range(left, right):
            p = frame.getpixel((x, y))
            if not p[3]:
                continue
            nx = round(cx + (x - cx) * width) + sway
            if 0 <= nx < frame.width and 0 <= dest < frame.height:
                out.putpixel((nx, dest), p)
                if width < 0.95 and 0 <= nx + 1 < frame.width and out.getpixel((nx + 1, dest))[3] == 0 and x + 1 < right and frame.getpixel((x + 1, y))[3]:
                    out.putpixel((nx + 1, dest), p)
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
        points = [(x, y) for x, y in pixels_where(frame, is_light) if y < 38]
        return centroid(points), points

    up_eye, _ = frame_eye(up)
    f0 = bulge(shifted(up, 0, -2), [(x, y - 2) for x, y in pixels_where(up, is_light) if y < 38], (255, 196, 196), (255, 255, 255))

    mid_eye, mid_points = frame_eye(mid)
    f1 = clear(tear(mid, mid_eye, 24, 0), mid_points, socket)
    f1 = shards(f1, (mid_eye[0], mid_eye[1] + 7), [(-8, 0), (-3, 4), (4, 2), (9, 5)])

    down_eye, down_points = frame_eye(down)

    flat_eye, flat_points = frame_eye(flat)
    brown = {(99, 44, 8), (198, 130, 57), (156, 85, 33)}

    def split_lantern(frame, eye):
        lantern = Image.new("RGBA", frame.size, (0, 0, 0, 0))
        body = frame.copy()
        for y in range(frame.height):
            for x in range(frame.width):
                p = frame.getpixel((x, y))
                if p[3] and (p[:3] in brown or (is_light(p) and y >= 38)):
                    lantern.putpixel((x, y), p)
                    body.putpixel((x, y), (0, 0, 0, 0))
        return body, lantern

    def put(canvas, piece, x, bottom):
        box = piece.getbbox()
        if not box:
            return
        crop = piece.crop(box)
        canvas.alpha_composite(crop, (max(0, x), max(0, bottom - crop.height)))

    down_body, down_lantern = split_lantern(clear(tear(down, down_eye, 12, 0), down_points, socket), down_eye)
    falling = down_body.rotate(-12, resample=Image.NEAREST)
    f2 = Image.new("RGBA", down.size, (0, 0, 0, 0))
    f2.alpha_composite(shifted(falling, 0, 10))
    put(f2, down_lantern, 14, 62)

    flat_body, flat_lantern = split_lantern(clear(tear(flat, flat_eye, 12, 0), flat_points, socket), flat_eye)
    ground = scaled(flat_body, 1.04, 0.62, 62)
    ground_box = ground.getbbox()
    f3 = Image.new("RGBA", down.size, (0, 0, 0, 0))
    f3.alpha_composite(ground)
    put(f3, flat_lantern, ground_box[2] - 14, 62)
    ground4 = scaled(darken(flat_body, (60, 12, 20), 0.12), 1.08, 0.46, 62)
    box4 = ground4.getbbox()
    f4 = Image.new("RGBA", down.size, (0, 0, 0, 0))
    f4.alpha_composite(ground4)
    put(f4, flat_lantern.rotate(90), box4[2] - 12, 62)
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
    cracked = draw_cracks(shifted(idle, 0, -1), (eye[0], eye[1] - 1), (60, 22, 18), 4)
    empty = clear(idle, eye_points, socket)
    f1 = burst(shifted(empty, 0, 1), (eye[0], eye[1] + 1), 3, 10, 9, GOO, 0.3)

    f2 = slump(empty, 31, 0.9, 0.94, 3, 1.0, 0.0)
    f3 = slump(empty, 31, 0.62, 0.8, 5, 1.6, 1.3)
    f4 = slump(darken(empty, (20, 10, 16), 0.12), 31, 0.4, 0.68, 4, 2.0, 2.6)
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
