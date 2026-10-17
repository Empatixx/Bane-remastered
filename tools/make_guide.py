"""The crypt's guide: a hooded keeper in a long robe with a lantern, who talks the player through
the training. Drawn by hand at the Scientist's pixel scale (16x40 per frame, six frames of idle:
a slow breath and the lantern's flame flickering).

    uv run -q --with pillow python tools/make_guide.py [preview.png]
"""

import sys

from PIL import Image

W, H, FRAMES = 16, 40, 6

PALETTE = {
    ".": None,
    "k": (24, 20, 30),  # outline and the dark inside the hood
    "h": (52, 44, 64),  # robe shadow
    "r": (78, 66, 92),  # robe
    "l": (108, 94, 124),  # robe light
    "s": (238, 195, 154),  # skin
    "d": (196, 150, 114),  # skin shadow
    "e": (250, 214, 120),  # eyes catching the lantern
    "b": (150, 112, 64),  # rope belt
    "B": (196, 156, 92),  # rope light
    "g": (104, 84, 52),  # lantern frame
    "y": (244, 196, 84),  # lantern glass
    "w": (255, 238, 170),  # flame
    "f": (40, 32, 28),  # feet
}

# One frame, top to bottom. The lantern hangs from the right hand.
BODY = [
    "................",
    ".....kkkk.......",
    "....krrrrk......",
    "...krllrrrk.....",
    "..krlrrrrrhk....",
    "..krlrrrrrhk....",
    "..krkkkkkkhk....",
    "..krkhhhhkhk....",
    "..krkeshekhk....",
    "..krkssssdhk....",
    "..krkdssddhk....",
    "..krrkddkrhk....",
    ".krllrkkrrrhk...",
    ".krlrrrrrrrhk...",
    "krlrrrrrrrrrhk..",
    "krlrrrrrrrrrhk..",
    "krlrrrrrrrrrhkg.",
    "krlrrrrrrrrrhsg.",
    "kslrrrrrrrrrssgg",
    "krlrrrrrrrrrhkgg",
    ".klrrrrrrrrhkgyg",
    ".kbBbbbbbbbbkgwg",
    ".klrrrbrrrrhkgyg",
    ".klrrrbrrrrhk.g.",
    ".klrrrbrrrrhk...",
    ".klrrrrrrrrhk...",
    ".klrrrrrrrrhk...",
    ".klrrrrrrrrhk...",
    "klrrrrrrrrrrhk..",
    "klrrrrrrrrrrhk..",
    "klrrrrrrrrrrhk..",
    "klrrrrrrrrrrhk..",
    "klrrrrrrrrrrhk..",
    "klrrrrrrrrrrhk..",
    "klrrrrrrrrrrhk..",
    "khrrrrrrrrrhhk..",
    "kkhhhhhhhhhhkk..",
    "..kffk..kffk....",
    "..kffk..kffk....",
    "................",
]


def frame(index: int) -> Image.Image:
    image = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    # A slow breath: the upper body rises a pixel in the middle frames.
    lift = 1 if index in (2, 3) else 0
    for row, line in enumerate(BODY):
        line = line[:W].ljust(W, ".")
        target = row - lift if row < 36 else row
        if target < 0:
            continue
        for column, key in enumerate(line):
            colour = PALETTE[key]
            if key == "w" and index % 2 == 1:
                colour = PALETTE["y"]
            if colour is not None:
                image.putpixel((column, target), (*colour, 255))
    return image


def main() -> None:
    sheet = Image.new("RGBA", (W * FRAMES, H), (0, 0, 0, 0))
    for index in range(FRAMES):
        sheet.paste(frame(index), (index * W, 0))
    if len(sys.argv) > 1:
        preview = Image.new("RGBA", sheet.size, (30, 28, 36, 255))
        preview.alpha_composite(sheet)
        preview.resize((sheet.width * 10, sheet.height * 10), Image.NEAREST).save(sys.argv[1])
        return
    sheet.save("assets/textures/ingame/guide.tga")


if __name__ == "__main__":
    main()
