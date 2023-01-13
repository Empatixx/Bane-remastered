package cz.empatix.assets.creators;

import cz.empatix.assets.domain.RawAudio;
import org.lwjgl.openal.AL10;

import java.util.Map;
import java.util.stream.Collectors;

public class AudioCreator implements AssetCreator<RawAudio, Integer> {
    private Map<String, Integer> audio;

    @Override
    public Integer get(final String name) {
        return audio.get(name);
    }

    @Override
    public void create(final Map<String, RawAudio> data) {
        audio = data.entrySet().stream()
                .map(entry -> Map.entry(entry.getKey(), createAudio(entry.getValue())))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
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
