package cz.empatix.assets.loaders;

import cz.empatix.assets.domain.RawShader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class ShadersLoader implements AssetLoader {
    private final static Logger LOGGER = LoggerFactory.getLogger(ShadersLoader.class);
    public final static Path PATH = Path.of("./assets/shaders/");
    private static final Path[] PRELOAD_SHADERS = {
            PATH.resolve("common-vs.glsl"),
            PATH.resolve("common-fs.glsl")
    };
    private final Map<String, RawShader> shaders;

    public ShadersLoader(final Map<String, RawShader> shaders) {
        this.shaders = shaders;
    }

    @Override
    public int load() {
        LOGGER.info("Loading shader files from {}", PATH);
        final long start = System.currentTimeMillis();
        final AtomicInteger total = new AtomicInteger();
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths
                    .filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".glsl"))
                    .collect(Collectors.groupingBy(p -> p.getFileName().toString().replaceAll("(-vs)*(-fs)*(\\.glsl)", "")))
                    .entrySet()
                    .stream()
                    .filter(entry -> !shaders.containsKey(entry.getKey()))
                    .map(this::createRawShader)
                    .forEach(entry -> {
                        shaders.put(entry.getKey(), entry.getValue());
                        total.getAndIncrement();
                    });
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            LOGGER.info("Loaded {} shaders in {} ms", total.get(), System.currentTimeMillis() - start);
        }
        return shaders.size();
    }

    @Override
    public int preload() {
        Arrays.stream(PRELOAD_SHADERS)
                .collect(Collectors.groupingBy(p -> p.getFileName().toString().replaceAll("(-vs)*(-fs)*(\\.glsl)", "")))
                .entrySet()
                .stream()
                .map(this::createRawShader)
                .forEach(entry -> shaders.put(entry.getKey(), entry.getValue()));
        return PRELOAD_SHADERS.length;
    }
    private Map.Entry<String, RawShader> createRawShader(final Map.Entry<String, List<Path>> entry) {
        final List<Path> value = entry.getValue();
        final String vs = getShaderText(value, "-vs.glsl")
                .orElseThrow(() -> new RuntimeException("Vertex shader not found!"));
        final String fs = getShaderText(value, "-fs.glsl")
                .orElseThrow(() -> new RuntimeException("Fragment shader not found!"));
        return Map.entry(entry.getKey(), new RawShader(vs, fs));
    }

    private Optional<String> getShaderText(final List<Path> paths, final String suffix) {
        return paths.stream()
                .filter(p -> p.toString().endsWith(suffix))
                .findFirst()
                .map(p -> {
                    try {
                        return new String(Files.readAllBytes(p));
                    } catch (IOException e) {
                        LOGGER.error("Error while reading shader file: {}", p);
                        throw new RuntimeException(e);
                    }
                });
    }

}
