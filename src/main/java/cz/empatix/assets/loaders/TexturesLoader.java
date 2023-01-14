package cz.empatix.assets.loaders;

import cz.empatix.assets.creators.ShadersCreator;
import cz.empatix.assets.domain.RawImage;
import cz.empatix.assets.creators.AssetCreator;
import cz.empatix.assets.creators.TexturesCreator;
import org.lwjgl.system.MemoryStack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.IntBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.Stream;

import static org.lwjgl.stb.STBImage.*;

public class TexturesLoader implements AssetLoader<RawImage> {
    private final static Logger LOGGER = LoggerFactory.getLogger(TexturesLoader.class);
    private final static String path = "./assets/textures/";
    private final Map<String, RawImage> textures;
    public TexturesLoader(){
        textures = new HashMap<>();
    }

    @Override
    public int load() {
        try (final Stream<Path> paths = Files.walk(Paths.get(path))) {
            paths.filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".tga") || p.toString().endsWith(".png"))
                    .parallel()
                    .map(path -> Map.entry(path.toString(), decodeImage(path)))
                    .forEach(entry -> this.textures.put(entry.getKey(), entry.getValue()));
            return textures.size();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public AssetCreator<RawImage, Integer> getCreator() {
        final AssetCreator<RawImage, Integer> creator = new TexturesCreator();
        creator.create(textures);
        return creator;
    }

    private RawImage decodeImage(final Path path) {
        try (MemoryStack stack = MemoryStack.stackPush()) {
            final IntBuffer w = stack.mallocInt(1);
            final IntBuffer h = stack.mallocInt(1);
            final IntBuffer channels = stack.mallocInt(1);

            /* Load image */
            final ByteBuffer buffer = stbi_load(path.toAbsolutePath().toString(), w, h, channels, 0);

            if (buffer == null) {
                throw new RuntimeException("Failed to load a texture file!" + System.lineSeparator() + stbi_failure_reason());
            }

            buffer.flip();

            return new RawImage(w.get(0), h.get(0), channels.get(0), buffer);
        } catch (Exception e) {
            LOGGER.error("Failed to load texture: " + path.toString(), e);
            throw new RuntimeException(e);
        }
    }

    @Override
    public AssetLoader<RawImage> call() throws Exception {
        LOGGER.info("Loading textures...");
        final long start = System.currentTimeMillis();
        final int loaded = load();
        LOGGER.info("Loaded {} textures in {} ms", loaded, System.currentTimeMillis() - start);
        return this;
    }
}
