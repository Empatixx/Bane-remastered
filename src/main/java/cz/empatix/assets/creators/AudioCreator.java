package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawAudio;
import org.lwjgl.openal.AL10;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;
import java.util.stream.Collectors;

public class AudioCreator implements AssetCreator<RawAudio, Integer> {
    private static final Logger LOGGER = LoggerFactory.getLogger(AudioCreator.class);
    private Map<String, Integer> audio;

    @Override
    public Integer get(final String name) {
        return audio.get(name);
    }

    @Override
    public void create(final Map<String, RawAudio> data) {
        LOGGER.info("Creating audio...");
        final long start = System.currentTimeMillis();
        audio = data.entrySet().stream()
                .map(entry -> Map.entry(entry.getKey(), createAudio(entry.getValue())))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        LOGGER.info("Audio created in {} ms", System.currentTimeMillis() - start);
    }
    private int createAudio(final RawAudio audio){
        final int buffer = AL10.alGenBuffers();
        AL10.alBufferData(buffer, audio.format(), audio.data(), audio.sampleRate());
        audio.destroy();
        return buffer;
    }

    @Override
    public void destroy() {
        audio.forEach((s, integer) -> {
            AL10.alDeleteBuffers(integer);
        });
    }
}
