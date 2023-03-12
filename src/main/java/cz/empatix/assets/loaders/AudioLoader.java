package cz.empatix.assets.loaders;

import cz.empatix.assets.domain.RawAudio;
import org.lwjgl.system.MemoryStack;
import org.lwjgl.system.MemoryUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.IntBuffer;
import java.nio.ShortBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Map;
import java.util.stream.Stream;

import static org.lwjgl.openal.AL10.AL_FORMAT_MONO16;
import static org.lwjgl.openal.AL10.AL_FORMAT_STEREO16;
import static org.lwjgl.stb.STBVorbis.stb_vorbis_decode_filename;
import static org.lwjgl.system.MemoryStack.stackPush;

public class AudioLoader implements AssetLoader {
    private final static Logger LOGGER = LoggerFactory.getLogger(AudioLoader.class);

    public final static Path PATH = Path.of("./assets/audio/");
    private final Map<String, RawAudio> audio;

    public AudioLoader(final Map<String, RawAudio> audio) {
        this.audio = audio;
    }


    @Override
    public int load() {
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths
                    .parallel()
                    .filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".ogg"))
                    .map(path -> Map.entry(path.toString(), decodeAudio(path)))
                    .forEach(entry -> this.audio.put(entry.getKey(), entry.getValue()));
            return audio.size();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public int preload() {
        return 0;
    }

    private RawAudio decodeAudio(final Path path) {
        try {
            try (MemoryStack stack = stackPush()) {
                //Allocate space to store return information from the function
                final IntBuffer channelsBuffer = stack.mallocInt(1);
                final IntBuffer sampleRateBuffer = stack.mallocInt(1);

                final ShortBuffer rawAudioBuffer = stb_vorbis_decode_filename(path.toString(), channelsBuffer, sampleRateBuffer);
                if (rawAudioBuffer == null) {
                    throw new RuntimeException("Failed to load audio file: " + path);
                }
                try{
                    final int sampleRate = sampleRateBuffer.get();
                    final int channels = channelsBuffer.get();
                    final int format = switch (channels) {
                        case 1 -> AL_FORMAT_MONO16;
                        case 2 -> AL_FORMAT_STEREO16;
                        default -> throw new RuntimeException("Audio file has unsupported number of channels: " + channels);
                    };
                    final short[] audioData = new short[rawAudioBuffer.remaining()];
                    rawAudioBuffer.get(audioData);

                    return new RawAudio(format, sampleRate, audioData);
                } finally {
                    MemoryUtil.memFree(rawAudioBuffer);
                }
            }
        } catch (Exception e) {
            LOGGER.error("Error while decoding audio file: " + path.toString(), e);
            throw new RuntimeException(e);
        }
    }

    @Override
    public Integer call(){
        final long start = System.currentTimeMillis();
        final int loaded = load();
        LOGGER.info("Loaded {} audio files in {} ms", loaded, System.currentTimeMillis() - start);
        return loaded;
    }
}
