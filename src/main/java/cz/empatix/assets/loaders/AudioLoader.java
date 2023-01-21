package cz.empatix.assets.loaders;

import cz.empatix.assets.creators.AudioCreator;
import cz.empatix.assets.domain.RawAudio;
import org.lwjgl.system.MemoryStack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.IntBuffer;
import java.nio.ShortBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.Stream;

import static org.lwjgl.openal.AL10.*;
import static org.lwjgl.stb.STBVorbis.stb_vorbis_decode_filename;
import static org.lwjgl.system.MemoryStack.stackPush;

public class AudioLoader implements AssetLoader<RawAudio> {
    private final static Logger LOGGER = LoggerFactory.getLogger(AudioLoader.class);

    public final static Path PATH = Path.of("./assets/audio/");
    private final Map<String, RawAudio> audio;
    private final AudioCreator audioCreator;
    public AudioLoader(){
        audio = new HashMap<>();
        audioCreator = new AudioCreator();
    }
    @Override
    public int load() {
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths.filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".ogg"))
                    .parallel()
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

    @Override
    public void submit() {
        audioCreator.add(audio);
    }

    @Override
    public AudioCreator getCreator() {
        return audioCreator;
    }


    private RawAudio decodeAudio(final Path path) {
        try{
            try (MemoryStack stack = stackPush()) {
                //Allocate space to store return information from the function
                final IntBuffer channelsBuffer   = stack.mallocInt(1);
                final IntBuffer sampleRateBuffer = stack.mallocInt(1);

                final ShortBuffer rawAudioBuffer = stb_vorbis_decode_filename(path.toString(), channelsBuffer, sampleRateBuffer);
                if(rawAudioBuffer == null){
                    throw new RuntimeException("Failed to load audio file: " + path);
                }

                //Retreive the extra information that was stored in the buffers by the function
                final int format;
                final int sampleRate = sampleRateBuffer.get(0);
                final int channels   = channelsBuffer.get(0);
                switch (channels) {
                    case 1:
                        format = AL_FORMAT_MONO16;
                        break;
                    case 2:
                        format = AL_FORMAT_STEREO16;
                        break;
                    default:
                        throw new RuntimeException("Audio file has unsupported number of channels: " + channels);
                }
                return new RawAudio(format, sampleRate, rawAudioBuffer);
            }
        } catch (Exception e) {
            LOGGER.error("Error while decoding audio file: " + path.toString(), e);
            throw new RuntimeException(e);
        }
    }

    @Override
    public AudioLoader call() throws Exception {
        final long start = System.currentTimeMillis();
        final int loaded = load();
        LOGGER.info("Loaded {} audio files in {} ms", loaded, System.currentTimeMillis() - start);
        audioCreator.ready();
        return this;
    }
}
