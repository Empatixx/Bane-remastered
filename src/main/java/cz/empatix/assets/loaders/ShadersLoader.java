package cz.empatix.assets.loaders;

import cz.empatix.assets.creators.AssetCreator;
import cz.empatix.assets.creators.AudioCreator;
import cz.empatix.assets.creators.ShadersCreator;
import cz.empatix.assets.domain.RawShader;
import cz.empatix.render.Shader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class ShadersLoader implements AssetLoader<RawShader> {
    private final static Logger LOGGER = LoggerFactory.getLogger(ShadersLoader.class);
    private final static String path = "./assets/shaders/";
    private final Map<String, RawShader> shaders;

    public ShadersLoader() {
        shaders = new HashMap<>();
    }

    @Override
    public int load() {
        try (final Stream<Path> paths = Files.walk(Paths.get(path))) {
            final Map<String, List<Path>> shaderPaths = paths.filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".glsl"))
                    .collect(Collectors.groupingBy(p -> p.getFileName().toString().replaceAll("(-vs)*(-fs)*(\\.glsl)", "")));
            shaderPaths.entrySet().stream()
                    .map(entry -> {
                        final List<Path> value = entry.getValue();
                        final String vs = getShaderText(value, "-vs.glsl")
                                .orElseThrow(() -> new RuntimeException("Vertex shader not found!"));
                        final String fs = getShaderText(value, "-fs.glsl")
                                .orElseThrow(() -> new RuntimeException("Fragment shader not found!"));
                        return Map.entry(entry.getKey(), new RawShader(vs, fs));
                    })
                    .forEach(entry -> this.shaders.put(entry.getKey(), entry.getValue()));
            return shaderPaths.size();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public AssetCreator<RawShader, Shader> getCreator() {
        final AssetCreator<RawShader, Shader> creator = new ShadersCreator();
        creator.create(shaders);
        return creator;
    }

    private Optional<String> getShaderText(final List<Path> paths, final String suffix) {
        return paths.stream()
                .filter(p -> p.toString().endsWith(suffix))
                .findFirst()
                .map(p -> {
                    try {
                        return new String(Files.readAllBytes(p));
                    } catch (IOException e) {
                        throw new RuntimeException(e);
                    }
                });
    }

    @Override
    public AssetLoader<RawShader> call() throws Exception {
        LOGGER.info("Loading shaders...");
        final long start = System.currentTimeMillis();
        final int loaded = load();
        LOGGER.info("Loaded {} shaders in {} ms", loaded, System.currentTimeMillis() - start);
        return this;
    }
}
