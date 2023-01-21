package cz.empatix.assets.service;

import cz.empatix.assets.creators.*;
import cz.empatix.assets.domain.Metadata;
import cz.empatix.assets.loaders.*;
import cz.empatix.render.domain.Shader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class AssetsService {
    private static List<Future<AssetLoader<?>>> futures;
    private static ExecutorService executorService;

    private static ShadersCreator shadersCreator;
    private static TexturesCreator texturesCreator;
    private static AudioCreator audioCreator;
    private static MetadataCreator metadataCreator;

    private final static Logger LOGGER = LoggerFactory.getLogger(AssetsService.class);

    public static void loadAssets() {
        executorService = Executors.newFixedThreadPool(4);

        final ShadersLoader shadersLoader = new ShadersLoader();
        final TexturesLoader texturesLoader = new TexturesLoader();
        final AudioLoader audioLoader = new AudioLoader();
        final MetadataLoader metadataLoader = new MetadataLoader();

        shadersCreator = shadersLoader.getCreator();
        texturesCreator = texturesLoader.getCreator();
        audioCreator = audioLoader.getCreator();
        metadataCreator = metadataLoader.getCreator();

        LOGGER.info("Preloading assets...");
        final long start = System.currentTimeMillis();
        Stream.of(shadersLoader, texturesLoader, metadataLoader).forEach(loader -> {
            loader.preload();
            loader.submit();
        });
        LOGGER.info("Preloading assets done in {} ms", System.currentTimeMillis() - start);


        futures = Stream.of(shadersLoader, texturesLoader, audioLoader, metadataLoader)
                .map(executorService::submit)
                .collect(Collectors.toList());
    }

    public static void createAssets() {
        futures.forEach(future -> {
            try {
                if (future.isDone()) {
                    final AssetLoader<?> loader = future.get();
                    loader.submit();
                    loader.getCreator().ready();
                }
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        });
        futures.removeIf(Future::isDone);
        if(isLoaded()){
            executorService.shutdownNow();
        }
    }

    public static boolean isLoaded() {
        return Stream.of(shadersCreator, texturesCreator, audioCreator, metadataCreator)
                .allMatch(AssetCreator::isReady);
    }

    public static void dispose() {
        if (!isLoaded()) return;
        List.of(shadersCreator, texturesCreator, audioCreator, metadataCreator).forEach(AssetCreator::destroy);
    }

    public static int getTexture(final String name) {
        final Integer texture = texturesCreator.get(name);
        if (texture == null) {
            LOGGER.error("Texture {} not found", name);
            throw new RuntimeException("Texture " + name + " not found");
        }
        return texture;
    }

    public static int getAudio(final String name) {
        final Integer texture = audioCreator.get(name);
        if (texture == null) {
            LOGGER.error("Audio {} not found", name);
            throw new RuntimeException("Audio " + name + " not found");
        }
        return texture;
    }

    public static Shader getShader(final String name) {
        final Shader shader = shadersCreator.get(name);
        if (shader == null) {
            LOGGER.error("Shader {} not found", name);
            throw new RuntimeException("Shader " + name + " not found");
        }
        return shader;
    }

    public static Metadata getMetadata(final String name) {
        final Metadata metadata = metadataCreator.get(name);
        if (metadata == null) {
            LOGGER.error("Metadata {} not found", name);
            throw new RuntimeException("Metadata " + name + " not found");
        }
        return metadata;
    }
}
