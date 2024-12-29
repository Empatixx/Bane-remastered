package cz.empatix.assets.service;

import cz.empatix.assets.providers.*;
import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.textureatlas.domain.Texture;
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
    private static final AudioProvider AUDIO_PROVIDER = new AudioProvider();
    private static final FontProvider FONT_PROVIDER = new FontProvider();
    private static final ShaderProgramProvider SHADER_PROGRAM_PROVIDER = new ShaderProgramProvider();
    private static final TexturesProvider TEXTURES_PROVIDER = new TexturesProvider();
    private static final NoiseTextureProvider NOISE_TEXTURE_PROVIDER = new NoiseTextureProvider();
    private static final ExecutorService executorService = Executors.newWorkStealingPool();
    private static final AtomicBoolean LOADED = new AtomicBoolean(false);
    private static List<? extends Future<?>> loadProcesses;

    public static void loadAssets() {
        LOGGER.info("Preloading assets...");
        final long start = System.currentTimeMillis();
        Stream.of(AUDIO_PROVIDER, FONT_PROVIDER, SHADER_PROGRAM_PROVIDER, TEXTURES_PROVIDER, NOISE_TEXTURE_PROVIDER).forEach(AssetProvider::preload);
        Stream.of(AUDIO_PROVIDER, FONT_PROVIDER, SHADER_PROGRAM_PROVIDER, TEXTURES_PROVIDER, NOISE_TEXTURE_PROVIDER).forEach(AssetProvider::create);
        LOGGER.info("Preloading assets done in {} ms", System.currentTimeMillis() - start);
        LOGGER.info("Loading assets...");
        loadProcesses = List.of(
                executorService.submit(AUDIO_PROVIDER::load),
                executorService.submit(FONT_PROVIDER::load),
                executorService.submit(SHADER_PROGRAM_PROVIDER::load),
                executorService.submit(TEXTURES_PROVIDER::load),
                executorService.submit(NOISE_TEXTURE_PROVIDER::load)
        );
    }

    public static void createAssets() {
        LOADED.set(loadProcesses.stream().allMatch(Future::isDone));
        Stream.of(AUDIO_PROVIDER, FONT_PROVIDER, SHADER_PROGRAM_PROVIDER, TEXTURES_PROVIDER, NOISE_TEXTURE_PROVIDER).forEach(AssetProvider::create);
        if (LOADED.get()) {
            executorService.shutdownNow();
        }
    }

    public static boolean getLoaded() {
        return LOADED.get();
    }

    public static void dispose() {
        if (!getLoaded()) return;
        Stream.of(AUDIO_PROVIDER, FONT_PROVIDER, SHADER_PROGRAM_PROVIDER, TEXTURES_PROVIDER, NOISE_TEXTURE_PROVIDER).forEach(AssetProvider::destroy);
    }

    public static TextureAtlas.Entry getTexture(final String name) {
        return TEXTURES_PROVIDER.get(name);
    }

    public static int getAudio(final String name) {
        return AUDIO_PROVIDER.get(name);
    }

    public static ShaderProgram getShader(final String name) {
        return SHADER_PROGRAM_PROVIDER.get(name);
    }

    public static Font getFont(final String name, final FontSize size) {
        return FONT_PROVIDER.get(name, size);
    }

    public static Texture getNoiseTexture(final String name){
        return NOISE_TEXTURE_PROVIDER.get(name);
    }
}
