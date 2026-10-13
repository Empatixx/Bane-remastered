package cz.empatix.audio.assets;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.atomic.AtomicBoolean;

/**
 * Loading facade for audio assets, owned by the audio module. Mirrors the lifecycle the
 * rendering layer uses for its own assets (preload + async load, then create on the main
 * thread) so the application can drive both in parallel during the loading screen.
 */
public final class AudioAssets {
    private static final Logger LOGGER = LoggerFactory.getLogger(AudioAssets.class);
    private static final AudioProvider AUDIO_PROVIDER = new AudioProvider();
    private static final ExecutorService EXECUTOR = Executors.newSingleThreadExecutor();
    private static final AtomicBoolean LOADED = new AtomicBoolean(false);
    private static Future<?> loadProcess;

    private AudioAssets() {
    }

    public static void loadAssets() {
        LOGGER.info("Preloading audio...");
        AUDIO_PROVIDER.preload();
        AUDIO_PROVIDER.create();
        loadProcess = EXECUTOR.submit(AUDIO_PROVIDER::load);
    }

    public static void createAssets() {
        LOADED.set(loadProcess != null && loadProcess.isDone());
        AUDIO_PROVIDER.create();
        if (LOADED.get()) {
            EXECUTOR.shutdownNow();
        }
    }

    public static boolean getLoaded() {
        return LOADED.get();
    }

    public static void dispose() {
        if (!getLoaded()) return;
        AUDIO_PROVIDER.destroy();
    }

    public static int getAudio(final String name) {
        return AUDIO_PROVIDER.get(name);
    }
}
