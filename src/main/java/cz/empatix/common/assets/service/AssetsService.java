package cz.empatix.common.assets.service;

import cz.empatix.common.assets.creators.*;
import cz.empatix.common.assets.domain.font.Font;
import cz.empatix.common.assets.domain.metadata.Metadata;
import cz.empatix.common.assets.loaders.*;
import cz.empatix.render.domain.Shader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Optional;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.stream.Collectors;

public class AssetsService {
    private static List<Future<AssetLoader>> futures;
    private static List<AssetCreator> creators;
    private static ExecutorService executorService;
    private final static Logger LOGGER = LoggerFactory.getLogger(AssetsService.class);

    public static void loadAssets() {
        executorService = Executors.newFixedThreadPool(4);

        final List<AssetLoader> loaders = List.of(
                new ShadersLoader(),
                new TexturesLoader(),
                new AudioLoader(),
                new MetadataLoader(),
                new FontLoader()
        );
        creators = loaders.stream()
                .map(AssetLoader::getCreator)
                .toList();
        LOGGER.info("Preloading assets...");
        final long start = System.currentTimeMillis();
        loaders.forEach(loader -> {
            loader.preload();
            loader.submit();
        });
        LOGGER.info("Preloading assets done in {} ms", System.currentTimeMillis() - start);
        futures = loaders.stream()
                .map(executorService::submit)
                .collect(Collectors.toList());
    }

    public static void createAssets() {
        futures.forEach(future -> {
            try {
                if (future.isDone()) {
                    final AssetLoader loader = future.get();
                    loader.submit();
                    loader.getCreator().ready();
                }
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        });
        futures.removeIf(future -> {
            try {
                return future.isDone() && future.get().getCreator().isReady();
            } catch (InterruptedException | ExecutionException e) {
                throw new RuntimeException(e);
            }
        });
        if (isLoaded()) {
            executorService.shutdownNow();
        }
    }

    public static boolean isLoaded() {
        return creators.stream()
                .allMatch(AssetCreator::isReady);
    }

    public static void dispose() {
        if (!isLoaded()) return;
        creators.forEach(AssetCreator::destroy);
    }

    public static int getTexture(final String name) {
        return Optional.of(getCreator(TexturesCreator.class))
                .map(TexturesCreator.class::cast)
                .map(creator -> creator.get(name))
                .orElseGet(() -> {
                    LOGGER.error("Texture {} not found", name);
                    throw new RuntimeException("Texture " + name + " not found");
                });
    }

    public static int getAudio(final String name) {
        return Optional.of(getCreator(TexturesCreator.class))
                .map(TexturesCreator.class::cast)
                .map(creator -> creator.get(name))
                .orElseGet(() -> {
                    LOGGER.error("Audio {} not found", name);
                    throw new RuntimeException("Audio " + name + " not found");
                });
    }

    public static Shader getShader(final String name) {
        return Optional.of(getCreator(ShadersCreator.class))
                .map(ShadersCreator.class::cast)
                .map(creator -> creator.get(name))
                .orElseGet(() -> {
                    LOGGER.error("Shader {} not found", name);
                    throw new RuntimeException("Shader " + name + " not found");
                });
    }

    public static Metadata getMetadata(final String name) {
        return Optional.of(getCreator(MetadataCreator.class))
                .map(MetadataCreator.class::cast)
                .map(creator -> creator.get(name))
                .orElseGet(() -> {
                    LOGGER.error("Metadata {} not found", name);
                    throw new RuntimeException("Metadata " + name + " not found");
                });
    }
    public static Font getFont(final String name, final int size){
        return Optional.of(getCreator(FontCreator.class))
                .map(FontCreator.class::cast)
                .map(creator -> creator.get(name, size))
                .orElseGet(() -> {
                    LOGGER.error("Font {} not found", name);
                    throw new RuntimeException("Font " + name + " not found");
                });
    }
    private static AssetCreator getCreator(final Class<?> clazz) {
        return creators.stream()
                .filter(clazz::isInstance)
                .findFirst()
                .orElseThrow(() -> new RuntimeException("AssetCreator " + clazz + " not found"));
    }
}
