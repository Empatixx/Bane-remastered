package cz.empatix.audio.assets;
import cz.empatix.core.AssetProvider;

import org.lwjgl.openal.AL10;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class AudioProvider implements AssetProvider {
    private static final Logger LOGGER = LoggerFactory.getLogger(AudioProvider.class);
    private final Map<String, Integer> audio;
    private final AudioLoader audioLoader;

    public AudioProvider() {
        this.audio = new HashMap<>();
        audioLoader = new AudioLoader();
    }

    public Integer get(final String name) {
        return audio.get(AudioLoader.PATH.resolve(name).toString());
    }
    @Override
    public void create(){
        int totalInited = 0;
        final long start = System.currentTimeMillis();
        while (!audioLoader.getAssets().isEmpty()) {
            final RawAudio rawAudio = audioLoader.getAssets().poll();
            audio.put(rawAudio.filepath().toString(), createAudio(rawAudio));
            totalInited++;
        }
        final long end = System.currentTimeMillis();
        LOGGER.info("Inited " + totalInited + " audio files in " + (end - start) + "ms");
    }
    private int createAudio(final RawAudio audio){
        final int buffer = AL10.alGenBuffers();
        AL10.alBufferData(buffer, audio.format(), audio.data(), audio.sampleRate());
        return buffer;
    }
    @Override
    public void load() {
        audioLoader.load();
    }

    @Override
    public void preload() {
        audioLoader.preload();
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying audio");
        audio.forEach((s, integer) -> {
            AL10.alDeleteBuffers(integer);
        });
    }
}
