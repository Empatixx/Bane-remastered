from collections import Counter

from PIL import Image

SOURCE = "assets/textures/loading/loading.tga"
OUT = "assets/textures/loading/bane_title.tga"


def main():
    image = Image.open(SOURCE).convert("RGBA")
    background = Counter(image.getdata()).most_common(1)[0][0]
    pixels = image.load()
    for y in range(image.height):
        for x in range(image.width):
            if pixels[x, y] == background:
                pixels[x, y] = (0, 0, 0, 0)
    box = image.getbbox()
    cropped = image.crop(box)
    cropped.save(OUT)
    print(background, box, cropped.size)


main()
