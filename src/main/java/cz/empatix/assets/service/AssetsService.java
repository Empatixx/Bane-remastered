package cz.empatix.assets.service;

import cz.empatix.assets.creators.*;
import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.render.domain.Shader;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Optional;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;

public class AssetsService {
    private final static Logger LOGGER = LoggerFactory.getLogger(AssetsService.class);
    private static List<AssetCreator> creators;
    private static List<? extends Future<?>> loadProcesses;
    private static ExecutorService executorService;
    private static boolean loaded;

    public static void loadAssets() {
        executorService = Executors.newWorkStealingPool();
        creators = List.of(
                new TexturesCreator(),
                new AudioCreator(),
                new ShadersCreator(),
                new FontCreator()
        );
        LOGGER.info("Preloading assets...");
        final long start = System.currentTimeMillis();
        creators.parallelStream()
                .peek(AssetCreator::preload)
                .sequential()
                .forEach(AssetCreator::create);
        LOGGER.info("Preloading assets done in {} ms", System.currentTimeMillis() - start);
        LOGGER.info("Loading assets...");
        loadProcesses = creators
                .stream()
                .map(creator -> executorService.submit(creator.loadAsync()))
                .toList();
    }

    public static void  createAssets() {
        loaded = loadProcesses.stream().allMatch(Future::isDone);
        creators.forEach(AssetCreator::create);
        if (loaded) {
            executorService.shutdownNow();
        }
    }

    public static boolean isLoaded() {
        return loaded;
    }

    public static void dispose() {
        if (!isLoaded()) return;
        creators.forEach(AssetCreator::destroy);
    }

    public static TextureAtlas.Entry getTexture(final String name) {
        return Optional.of(getCreator(TexturesCreator.class))
                .map(creator -> creator.get(name))
                .orElseGet(() -> {
                    LOGGER.error("Texture {} not found", name);
                    throw new RuntimeException("Texture " + name + " not found");
                });
    }

    public static int getAudio(final String name) {
        return Optional.of(getCreator(AudioCreator.class))
                .map(creator -> creator.get(name))
                .orElseGet(() -> {
                    LOGGER.error("Audio {} not found", name);
                    throw new RuntimeException("Audio " + name + " not found");
                });
    }

    public static Shader getShader(final String name) {
        return Optional.of(getCreator(ShadersCreator.class))
                .map(creator -> creator.get(name))
                .orElseGet(() -> {
                    LOGGER.error("Shader {} not found", name);
                    throw new RuntimeException("Shader " + name + " not found");
                });
    }

    public static Font getFont(final String name, final FontSize size){
        return Optional.of(getCreator(FontCreator.class))
                .map(creator -> creator.get(name, size))
                .orElseGet(() -> {
                    LOGGER.error("Font {} not found", name);
                    throw new RuntimeException("Font " + name + " not found");
                });
    }

    private static <E extends AssetCreator> E getCreator(final Class<E> clazz) {
        return creators.stream()
                .filter(clazz::isInstance)
                .findFirst()
                .map(clazz::cast)
                .orElseThrow(() -> new RuntimeException("AssetCreator " + clazz + " not found"));
    }
}
