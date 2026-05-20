package cz.empatix.textureatlas.service;

import cz.empatix.assets.domain.RawImage;
import cz.empatix.core.GraphicsCapabilities;
import cz.empatix.textureatlas.domain.RawTextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlasBlueprint;
import cz.empatix.textureatlas.domain.TextureAtlasInfo;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class TextureAtlasPacker {
    private final Logger LOGGER = LoggerFactory.getLogger(TextureAtlasPacker.class);
    private static final int TEXTURE_MINSTEPSIZE = 128;
    private static final int MAX_TEXTURE_SIZE = 23170;
    private static final int BORDER_SIZE = 1; // Added border size constant

    public TextureAtlasBlueprint pack(final Map<String, RawImage> images) {
        TextureAtlasBlueprint lastSmallestAtlas = null;
        int targetSize = Math.min(GraphicsCapabilities.maxTextureSize, MAX_TEXTURE_SIZE);
        boolean failed;
        for (int stepSize = targetSize / 2; stepSize >= TEXTURE_MINSTEPSIZE; stepSize = stepSize / 2) {
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
                if (GraphicsCapabilities.maxTextureSize < targetSize) {
                    throw new IllegalStateException("Failed to pack images into atlas, graphics card limit is " + GraphicsCapabilities.maxTextureSize + "x" + GraphicsCapabilities.maxTextureSize);
                }
                targetSize += stepSize;
            } else {
                targetSize -= stepSize;
            }

        }
        return lastSmallestAtlas;
    }

    public RawTextureAtlas pack(final Map<String, RawImage> images,
                                final TextureAtlasBlueprint blueprint) {
        final byte[] atlas = new byte[blueprint.targetSize() * blueprint.targetSize() * 4];
        final Map<String, TextureAtlasInfo> atlasInfoMap = new HashMap<>();
        images.forEach((name, image) -> {
            final int x = blueprint.getX(name);
            final int y = blueprint.getY(name);
            addImageToAtlas(atlas, image.channels(), blueprint.targetSize(), image, x, y);
            atlasInfoMap.put(name, new TextureAtlasInfo(x, y, image.width(), image.height()));
        });
        return new RawTextureAtlas(atlas, blueprint.targetSize(), blueprint.targetSize(), atlasInfoMap);
    }

    private TextureAtlasBlueprint tryPack(final Map<String, RawImage> images,
                                          final int targetSize) {
        final TextureAtlasBlueprint blueprint = new TextureAtlasBlueprint(targetSize);
        final Queue<Rect> emptyRects = new PriorityQueue<>(images.size(), Comparator.comparingInt(Rect::height).thenComparing(Rect::width));
        emptyRects.add(new Rect(0, 0, targetSize, targetSize));
        images.forEach((name, image) -> {
            final Rect rect = emptyRects
                    .stream()
                    .filter(r -> r.canBeFilled(image, BORDER_SIZE))
                    .findFirst()
                    .orElseThrow(() -> new IllegalStateException("No empty rect found"));
            blueprint.addTexture(name, rect.x, rect.y);
            emptyRects.remove(rect);
            final int x = rect.x;
            final int y = rect.y;
            final int textureWidth = image.width();
            final int textureHeight = image.height();
            final int remainingWidth = rect.width - textureWidth - BORDER_SIZE;
            final int remainingHeight = rect.height - textureHeight - BORDER_SIZE;

            // Add the remaining empty rects with the border size included
            if (remainingHeight > 0) {
                emptyRects.add(new Rect(x, y + textureHeight + BORDER_SIZE, rect.width, remainingHeight));
            }
            if (remainingWidth > 0) {
                emptyRects.add(new Rect(x + textureWidth + BORDER_SIZE, y, remainingWidth, textureHeight));
            }
        });
        return blueprint;
    }

    private void addImageToAtlas(final byte[] data,
                                 final int channels,
                                 final int targetSize,
                                 final RawImage image,
                                 final int x,
                                 final int y) {
        switch (channels) {
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
            case TextureAtlas.R_CHANNELS -> {
                final int width = image.width();
                final int height = image.height();
                final byte[] imageData = image.pixels();

                for (int i = 0; i < height; i++) {
                    final int sourceOffset = i * width;
                    int targetOffset = ((y + i) * targetSize + x) * TextureAtlas.RGBA_CHANNELS;

                    for (int j = 0; j < width; j++) {
                        byte redValue = imageData[sourceOffset + j];
                        data[targetOffset + j * TextureAtlas.RGBA_CHANNELS] = redValue; // R
                        data[targetOffset + j * TextureAtlas.RGBA_CHANNELS + 1] = 0; // G
                        data[targetOffset + j * TextureAtlas.RGBA_CHANNELS + 2] = 0; // B
                        data[targetOffset + j * TextureAtlas.RGBA_CHANNELS + 3] = (byte) 255; // A
                    }
                }
            }
            default -> LOGGER.error("Unsupported number of channels: {}", channels);
        }
    }


    private record Rect(
            int x,
            int y,
            int width,
            int height
    ) {

        public boolean canBeFilled(final RawImage rawImage, final int borderSize) {
            return rawImage.width() <= width - borderSize && rawImage.height() <= height - borderSize;
        }
    }

}
