package cz.empatix.assets.service;

import cz.empatix.assets.creators.AssetCreator;
import cz.empatix.assets.creators.AudioCreator;
import cz.empatix.assets.creators.ShadersCreator;
import cz.empatix.assets.creators.TexturesCreator;
import cz.empatix.assets.loaders.AssetLoader;
import cz.empatix.assets.loaders.AudioLoader;
import cz.empatix.assets.loaders.ShadersLoader;
import cz.empatix.assets.loaders.TexturesLoader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class AssetsManager {
    private final static AssetsManager am = new AssetsManager();
    private static List<Future<AssetLoader<?>>> futures;

    public static AssetsManager getInstance() {
        return am;
    }

    private static ShadersCreator shadersCreator;
    private static TexturesCreator texturesCreator;
    private static AudioCreator audioCreator;

    private final static Logger LOGGER = LoggerFactory.getLogger(AssetsManager.class);

    public static void loadAssets() {
        final ExecutorService executorService = Executors.newFixedThreadPool(3);

        final ShadersLoader shadersLoader = new ShadersLoader();
        final TexturesLoader texturesLoader = new TexturesLoader();
        final AudioLoader audioLoader = new AudioLoader();

        futures = Stream.of(shadersLoader, texturesLoader, audioLoader)
                .map(executorService::submit)
                .collect(Collectors.toList());
    }

    public static void createAssets() {
        futures.forEach(future -> {
            try {
                if (future.isDone()) {
                    final AssetLoader<?> loader = future.get();
                    if (loader instanceof ShadersLoader) {
                        shadersCreator = (ShadersCreator) loader.getCreator();
                    } else if (loader instanceof TexturesLoader) {
                        texturesCreator = (TexturesCreator) loader.getCreator();
                    } else if (loader instanceof AudioLoader) {
                        audioCreator = (AudioCreator) loader.getCreator();
                    }
                }
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        });
        futures.removeIf(Future::isDone);
    }

    public static boolean isLoaded() {
        return shadersCreator != null && texturesCreator != null && audioCreator != null;
    }
    public static void dispose() {
        List.of(shadersCreator, texturesCreator, audioCreator).forEach(AssetCreator::destroy);
    }
}
