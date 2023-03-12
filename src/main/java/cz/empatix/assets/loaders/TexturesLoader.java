package cz.empatix.assets.loaders;

import cz.empatix.assets.creators.TexturesCreator;
import cz.empatix.assets.domain.RawImage;
import cz.empatix.textureatlas.domain.TextureAtlas;
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
    private final TexturesCreator texturesCreator;
    private final List<TextureAtlas> textureAtlases;

    public TexturesLoader() {
        textureAtlases = new ArrayList<>();
        this.texturesCreator = new TexturesCreator();
    }

    @Override
    public int load() {
        try (final Stream<Path> paths = Files.walk(PATH)) {
            final Map<String, RawImage> textures = paths.filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".tga") || p.toString().endsWith(".png"))
                    .parallel()
                    .map(path -> Map.entry(path.toString(), decodeImage(path)))
                    .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));

        } catch (IOException e) {
            LOGGER.error("Failed to load textures", e);
            throw new RuntimeException(e);
        }
        return textureAtlases.size();
    }

    @Override
    public int preload() {
        Arrays.stream(PRELOAD_TEXTURES)
                .map(path -> Map.entry(path.toString(), decodeImage(path)))
                .forEach(entry -> this.textureAtlases.put(entry.getKey(), entry.getValue()));
        return PRELOAD_TEXTURES.length;
    }

    @Override
    public void submit() {
        texturesCreator.add(textureAtlases);
    }

    @Override
    public TexturesCreator getCreator() {
        return texturesCreator;
    }

    protected RawImage decodeImage(final Path path) {
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

    @Override
    public TexturesLoader call() throws Exception {
        final long start = System.currentTimeMillis();
        final int loaded = load();
        LOGGER.info("Loaded {} textures in {} ms", loaded, System.currentTimeMillis() - start);
        return this;
    }
}
