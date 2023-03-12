package cz.empatix.assets.loaders;

import cz.empatix.assets.domain.RawImage;
import cz.empatix.textureatlas.domain.RawTextureAtlas;
import cz.empatix.textureatlas.factory.TextureAtlasFactory;
import org.lwjgl.system.MemoryStack;
import org.lwjgl.system.MemoryUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.IntBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static org.lwjgl.stb.STBImage.stbi_failure_reason;
import static org.lwjgl.stb.STBImage.stbi_load;

public class TexturesLoader implements AssetLoader {
    private final static Logger LOGGER = LoggerFactory.getLogger(TexturesLoader.class);
    public final static Path PATH = Path.of("./assets/textures/");
    private final static Path[] PRELOAD_TEXTURES = {
            PATH.resolve("Menu/loading.tga")
    };
    private final Queue<RawTextureAtlas> rawTextureAtlases;
    private final TextureAtlasFactory textureAtlasFactory;

    public TexturesLoader(final Queue<RawTextureAtlas> rawTextureAtlases){
        this.rawTextureAtlases = rawTextureAtlases;
        this.textureAtlasFactory = new TextureAtlasFactory();
    }

    @Override
    public int load() {
        LOGGER.info("Loading textures from {}", PATH);
        final AtomicInteger total = new AtomicInteger();
        final long start = System.currentTimeMillis();
        try (final Stream<Path> paths = Files.walk(PATH)) {
            final Map<String, RawImage> textures = paths
                    .parallel()
                    .filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".tga") || p.toString().endsWith(".png"))
                    .map(path -> Map.entry(path.toString(), decodeImage(path)))
                    .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
            final RawTextureAtlas textureAtlas = textureAtlasFactory.createAtlas(textures);
            rawTextureAtlases.add(textureAtlas);
            total.addAndGet(textures.size());
        } catch (IOException e) {
            LOGGER.error("Failed to load textures", e);
            throw new RuntimeException(e);
        } finally {
            LOGGER.info("Loaded {} textures in {} ms", total.get(), System.currentTimeMillis() - start);
        }
        return rawTextureAtlases.size();
    }

    @Override
    public int preload() {
        final Map<String, RawImage> loadingAtlas = Arrays.stream(PRELOAD_TEXTURES)
                .map(path -> Map.entry(path.toString(), decodeImage(path)))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        final RawTextureAtlas textureAtlas = textureAtlasFactory.createAtlas(loadingAtlas);
        rawTextureAtlases.add(textureAtlas);
        return PRELOAD_TEXTURES.length;
    }
    private RawImage decodeImage(final Path path) {
        try (MemoryStack stack = MemoryStack.stackPush()) {
            final IntBuffer pWidth = stack.mallocInt(1);
            final IntBuffer pHeight = stack.mallocInt(1);
            final IntBuffer pChannels = stack.mallocInt(1);

            /* Load image */
            final ByteBuffer buffer = stbi_load(path.toAbsolutePath().toString(), pWidth, pHeight, pChannels, 0);

            if (buffer == null) {
                throw new RuntimeException("Failed to load a texture file!" + System.lineSeparator() + stbi_failure_reason());
            }
            try{
                final int width = pWidth.get();
                final int height = pHeight.get();
                final int channels = pChannels.get();
                final byte[] pixels = new byte[width * height * channels];
                buffer.get(pixels);

                return new RawImage(width, height, channels, pixels);
            } finally {
                MemoryUtil.memFree(buffer);
            }
        } catch (Exception e) {
            LOGGER.error("Failed to load texture: " + path.toString(), e);
            throw new RuntimeException(e);
        }
    }
}
