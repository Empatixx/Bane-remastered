package cz.empatix.textureatlas.service;

import cz.empatix.assets.domain.RawImage;
import cz.empatix.textureatlas.domain.RawTextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlasInfo;
import org.lwjgl.opengl.GL30;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

import static org.lwjgl.opengl.GL11C.GL_MAX_TEXTURE_SIZE;

public class TextureAtlasPacker {
    private final Logger LOGGER = LoggerFactory.getLogger(TextureAtlasPacker.class);
    private static final int TEXTURE_MINSTEPSIZE = 128;
    private static final int MAX_TEXTURE_SIZE = 23170;
    private final int maxAtlasSize;

    public TextureAtlasPacker() {
        final int maxGraphicsTextureSize = GL30.glGetInteger(GL_MAX_TEXTURE_SIZE);
        this.maxAtlasSize = Math.min(maxGraphicsTextureSize, MAX_TEXTURE_SIZE);
    }

    public RawTextureAtlas pack(final Map<String, RawImage> images) {
        RawTextureAtlas lastSmallestAtlas = null;
        int targetSize = maxAtlasSize;
        boolean failed;
        for (int stepSize = maxAtlasSize / 2; stepSize >= TEXTURE_MINSTEPSIZE; stepSize = stepSize / 2) {
            try {
                final long startTime = System.currentTimeMillis();
                lastSmallestAtlas = tryPack(images, targetSize);
                LOGGER.info("Packed {} images into {}x{} atlas in {} ms", images.size(), targetSize, targetSize, System.currentTimeMillis() - startTime);
                failed = false;
            } catch (final IllegalStateException e) {
                LOGGER.info("Failed to pack {} images into {}x{} atlas", images.size(), targetSize, targetSize);
                failed = true;
            }

            if (failed) {
                if (maxAtlasSize <= targetSize) {
                    throw new IllegalStateException("Failed to pack images into atlas, graphics card limit is " + maxAtlasSize + "x" + maxAtlasSize);
                }
                targetSize += stepSize;
            } else {
                targetSize -= stepSize;
            }

        }
        return lastSmallestAtlas;
    }

    private RawTextureAtlas tryPack(final Map<String, RawImage> images,
                                    final int targetSize) {
        final Map<String, TextureAtlasInfo> textureInfos = new HashMap<>();
        final byte[] data = new byte[targetSize * targetSize * 4];

        final Queue<Rect> emptyRects = new PriorityQueue<>(images.size(), Comparator.comparingInt(Rect::height).thenComparing(Rect::width));
        emptyRects.add(new Rect(0, 0, targetSize, targetSize));
        images.forEach((name, image) -> {
            final Rect rect = emptyRects
                    .stream()
                    .filter(r -> r.canBeFilled(image))
                    .findFirst()
                    .orElseThrow(() -> new IllegalStateException("No empty rect found"));
            textureInfos.put(name, new TextureAtlasInfo(rect.x, rect.y, image.width(), image.height()));
            emptyRects.remove(rect);
            final int x = rect.x;
            final int y = rect.y;
            final int textureWidth = image.width();
            final int textureHeight = image.height();
            final int remainingWidth = rect.width - textureWidth;
            final int remainingHeight = rect.height - textureHeight;
            if (remainingHeight > 0) {
                emptyRects.add(new Rect(x, y + textureHeight, rect.width, remainingHeight));
            }
            if (remainingWidth > 0) {
                emptyRects.add(new Rect(x + textureWidth, y, remainingWidth, textureHeight));
            }
            addImageToAtlas(data, image.channels(), targetSize, image, x, y);

        });

        return new RawTextureAtlas(data, targetSize, targetSize, textureInfos);
    }

    private void addImageToAtlas(final byte[] data,
                                 final int channels,
                                 final int targetSize,
                                 final RawImage image,
                                 final int x,
                                 final int y) {
        switch (channels){
            case TextureAtlas.RGBA_CHANNELS -> {
                final int width = image.width();
                final int height = image.height();
                final byte[] imageData = image.pixels();

                for (int i = 0; i < height; i++) {
                    System.arraycopy(imageData, i * width * TextureAtlas.RGBA_CHANNELS, data, ((y + i) * targetSize + x) * TextureAtlas.RGBA_CHANNELS, width * TextureAtlas.RGBA_CHANNELS);
                }
            }
            case TextureAtlas.RGB_CHANNELS -> {
                final int width = image.width();
                final int height = image.height();
                final byte[] imageData = image.pixels();

                for (int i = 0; i < height; i++) {
                    int sourceOffset = i * width * 3;
                    int targetOffset = ((y + i) * targetSize + x) * 4;

                    for (int j = 0; j < width; j++) {
                        System.arraycopy(imageData, sourceOffset + j * TextureAtlas.RGB_CHANNELS, data, targetOffset + j * TextureAtlas.RGBA_CHANNELS, TextureAtlas.RGB_CHANNELS);
                    }

                    for (int j = 0; j < width; j++) {
                        data[targetOffset + j * TextureAtlas.RGBA_CHANNELS + TextureAtlas.RGB_CHANNELS] = (byte) 255;
                    }
                }
            }
            default -> throw new IllegalStateException("Unsupported image channels: " + channels);
        }
    }


    private record Rect(
            int x,
            int y,
            int width,
            int height
    ) {

        public boolean canBeFilled(final RawImage rawImage) {
            return rawImage.width() <= width && rawImage.height() <= height;
        }
    }

}
