package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawAudio;
import cz.empatix.assets.loaders.AudioLoader;
import org.lwjgl.openal.AL10;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;

public class AudioCreator implements AssetCreator{
    private static final Logger LOGGER = LoggerFactory.getLogger(AudioCreator.class);
    private boolean ready;
    private final Map<String, Integer> audio;

    public AudioCreator() {
        this.audio = new HashMap<>();
    }

    public Integer get(final String name) {
        return audio.get(AudioLoader.PATH.resolve(name).toString());
    }

    public void add(final Map<String, RawAudio> data) {
        final long start = System.currentTimeMillis();
        final int loaded = audio.size();
        data.entrySet().stream()
                .map(entry -> Map.entry(entry.getKey(), createAudio(entry.getValue())))
                .forEach(entry -> audio.put(entry.getKey(), entry.getValue()));
        LOGGER.info("Total {} audio vbos created in {} ms", audio.size() - loaded,  System.currentTimeMillis() - start);
    }
    private int createAudio(final RawAudio audio){
        final int buffer = AL10.alGenBuffers();
        AL10.alBufferData(buffer, audio.format(), audio.data(), audio.sampleRate());
        return buffer;
    }
    @Override
    public boolean isReady() {
        return ready;
    }

    @Override
    public void ready() {
        this.ready = true;
    }
    @Override
    public void destroy() {
        LOGGER.info("Destroying audio");
        audio.forEach((s, integer) -> {
            AL10.alDeleteBuffers(integer);
        });
    }
}
