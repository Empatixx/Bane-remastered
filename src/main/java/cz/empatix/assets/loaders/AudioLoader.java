package cz.empatix.assets.loaders;

import cz.empatix.assets.domain.RawAudio;
import cz.empatix.assets.domain.font.RawFont;
import org.lwjgl.system.MemoryStack;
import org.lwjgl.system.MemoryUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.IntBuffer;
import java.nio.ShortBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;
import java.util.stream.Stream;

import static org.lwjgl.openal.AL10.AL_FORMAT_MONO16;
import static org.lwjgl.openal.AL10.AL_FORMAT_STEREO16;
import static org.lwjgl.stb.STBVorbis.stb_vorbis_decode_filename;
import static org.lwjgl.system.MemoryStack.stackPush;

public class AudioLoader implements AssetLoader<RawAudio> {
    private final static Logger LOGGER = LoggerFactory.getLogger(AudioLoader.class);
    public final static Path PATH = Path.of("./assets/audio/");
    private final Queue<RawAudio> audio;
    private final Lock lock;
    public AudioLoader() {
        this.audio = new ConcurrentLinkedQueue<>();
        this.lock = new ReentrantLock();
    }


    @Override
    public int load() {
        LOGGER.info("Loading audio files from {}", PATH);
        final long start = System.currentTimeMillis();
        final AtomicInteger total = new AtomicInteger();
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths
                    .filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".ogg") || p.toString().endsWith(".wav"))
                    .map(this::decodeAudio)
                    .forEach(audio -> {
                        this.audio.add(audio);
                        total.incrementAndGet();
                    });
            return total.get();
        } catch (IOException e) {
            LOGGER.error("Failed to load audio files", e);
            throw new RuntimeException(e);
        } finally {
            LOGGER.info("Loaded {} audio files in {} ms", total.get(), System.currentTimeMillis() - start);
        }
    }

    @Override
    public int preload() {
        return 0;
    }

    @Override
    public Queue<RawAudio> getAssets() {
        return audio;
    }

    private RawAudio decodeAudio(final Path path) {
        try {
            try (MemoryStack stack = stackPush()) {
                //Allocate space to store return information from the function
                final IntBuffer channelsBuffer = stack.mallocInt(1);
                final IntBuffer sampleRateBuffer = stack.mallocInt(1);

                final ShortBuffer rawAudioBuffer = stb_vorbis_decode_filename(path.toString(), channelsBuffer, sampleRateBuffer);
                try{
                    if (rawAudioBuffer == null) {
                        throw new RuntimeException("Failed to load audio file: " + path);
                    }
                    final int sampleRate = sampleRateBuffer.get();
                    final int channels = channelsBuffer.get();
                    final int format = switch (channels) {
                        case 1 -> AL_FORMAT_MONO16;
                        case 2 -> AL_FORMAT_STEREO16;
                        default -> throw new RuntimeException("Audio file has unsupported number of channels: " + channels);
                    };
                    final short[] audioData = new short[rawAudioBuffer.remaining()];
                    rawAudioBuffer.get(audioData);

                    return new RawAudio(path, format, sampleRate, audioData);
                } finally {
                    MemoryUtil.memFree(rawAudioBuffer);
                }
            }
        } catch (Exception e) {
            LOGGER.error("Error while decoding audio file: " + path.toString(), e);
            throw new RuntimeException(e);
        }
    }
}
