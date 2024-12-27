package cz.empatix.assets.service;

import cz.empatix.assets.providers.*;
import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.stream.Stream;

public class AssetsService {
    private final static Logger LOGGER = LoggerFactory.getLogger(AssetsService.class);
    private static final AudioProvider audioProvider = new AudioProvider();
    private static final FontProvider fontProvider = new FontProvider();
    private static final ShaderProgramProvider shaderProgramProvider = new ShaderProgramProvider();
    private static final TexturesProvider texturesProvider = new TexturesProvider();
    private static final ExecutorService executorService = Executors.newWorkStealingPool();
    private static final AtomicBoolean LOADED = new AtomicBoolean(false);
    private static List<? extends Future<?>> loadProcesses;

    public static void loadAssets() {
        LOGGER.info("Preloading assets...");
        final long start = System.currentTimeMillis();
        Stream.of(audioProvider, fontProvider, shaderProgramProvider, texturesProvider).forEach(AssetProvider::preload);
        Stream.of(audioProvider, fontProvider, shaderProgramProvider, texturesProvider).forEach(AssetProvider::create);
        LOGGER.info("Preloading assets done in {} ms", System.currentTimeMillis() - start);
        LOGGER.info("Loading assets...");
        loadProcesses = List.of(
                executorService.submit(audioProvider::load),
                executorService.submit(fontProvider::load),
                executorService.submit(shaderProgramProvider::load),
                executorService.submit(texturesProvider::load)
        );
    }

    public static void  createAssets() {
        LOADED.set(loadProcesses.stream().allMatch(Future::isDone));
        Stream.of(audioProvider, fontProvider, shaderProgramProvider, texturesProvider).forEach(AssetProvider::create);
        if (LOADED.get()) {
            executorService.shutdownNow();
        }
    }

    public static boolean getLoaded() {
        return LOADED.get();
    }

    public static void dispose() {
        if (!getLoaded()) return;
        Stream.of(audioProvider, fontProvider, shaderProgramProvider, texturesProvider).forEach(AssetProvider::destroy);
    }

    public static TextureAtlas.Entry getTexture(final String name) {
        return texturesProvider.get(name);
    }

    public static int getAudio(final String name) {
        return audioProvider.get(name);
    }

    public static ShaderProgram getShader(final String name) {
        return shaderProgramProvider.get(name);
    }

    public static Font getFont(final String name, final FontSize size){
        return fontProvider.get(name, size);
    }
}
