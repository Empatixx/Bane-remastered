package cz.empatix.assets.loaders;
import cz.empatix.core.AssetLoader;

import cz.empatix.assets.domain.RawShader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class ShadersLoader implements AssetLoader<List<RawShader>> {
    private final static Logger LOGGER = LoggerFactory.getLogger(ShadersLoader.class);
    public final static Path PATH = Path.of("./assets/shaders/");
    private static final Set<Path> PRELOAD_SHADERS = Set.of(
            PATH.resolve("common-vs.glsl"),
            PATH.resolve("common-fs.glsl"),
            PATH.resolve("hudCommon-vs.glsl"),
            PATH.resolve("hudCommon-fs.glsl")
    );
    private final Queue<List<RawShader>> shaders;
    public ShadersLoader() {
        this.shaders = new ConcurrentLinkedQueue<>();
    }

    @Override
    public int load() {
        LOGGER.info("Loading shader files from {}", PATH);
        final long start = System.currentTimeMillis();
        final AtomicInteger total = new AtomicInteger();
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths
                    .filter(Files::isRegularFile)
                    .filter(p -> !PRELOAD_SHADERS.contains(p))
                    .filter(p -> p.toString().endsWith(".glsl"))
                    .map(p -> {
                        try {
                            return new RawShader(p, new String(Files.readAllBytes(p)));
                        } catch (IOException e) {
                            LOGGER.error("Error while reading shader file: {}", p);
                            throw new RuntimeException(e);
                        }
                    })
                    .collect(Collectors.groupingBy(RawShader::getName))
                    .forEach((name, rawShaders) -> {
                        shaders.add(rawShaders);
                        total.incrementAndGet();
                    });
        } catch (IOException e) {
            LOGGER.error("Failed to load shader files", e);
            throw new RuntimeException(e);
        } finally {
            LOGGER.info("Loaded {} shaders in {} ms", total.get(), System.currentTimeMillis() - start);
        }
        return shaders.size();
    }

    @Override
    public int preload() {
        PRELOAD_SHADERS.stream()
                .map(p -> {
                    try {
                        return new RawShader(p, new String(Files.readAllBytes(p)));
                    } catch (IOException e) {
                        LOGGER.error("Error while reading shader file: {}", p);
                        throw new RuntimeException(e);
                    }
                })
                .collect(Collectors.groupingBy(RawShader::getName))
                .forEach((name, rawShaders) -> {
                    shaders.add(rawShaders);
                });
        return PRELOAD_SHADERS.size();
    }

    @Override
    public Queue<List<RawShader>> getAssets() {
        return shaders;
    }

}
