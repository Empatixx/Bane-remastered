package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawAudio;
import cz.empatix.assets.loaders.AudioLoader;
import org.lwjgl.openal.AL10;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.Callable;
import java.util.concurrent.ConcurrentHashMap;

public class AudioCreator implements AssetCreator{
    private static final Logger LOGGER = LoggerFactory.getLogger(AudioCreator.class);
    private final Map<String, Integer> audio;
    private final Map<String, RawAudio> rawAudio;
    private final AudioLoader audioLoader;

    public AudioCreator() {
        this.audio = new HashMap<>();
        this.rawAudio = new ConcurrentHashMap<>();
        audioLoader = new AudioLoader(rawAudio);
    }

    public Integer get(final String name) {
        return audio.get(AudioLoader.PATH.resolve(name).toString());
    }
    @Override
    public void create(){
        rawAudio.entrySet().stream()
                .map(entry -> Map.entry(entry.getKey(), createAudio(entry.getValue())))
                .forEach(entry -> audio.put(entry.getKey(), entry.getValue()));
    }
    private int createAudio(final RawAudio audio){
        final int buffer = AL10.alGenBuffers();
        AL10.alBufferData(buffer, audio.format(), audio.data(), audio.sampleRate());
        return buffer;
    }
    @Override
    public Callable<Void> load() {
        return audioLoader;
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying audio");
        audio.forEach((s, integer) -> {
            AL10.alDeleteBuffers(integer);
        });
    }
}
