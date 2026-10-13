package cz.empatix.audio.assets;


import java.nio.file.Path;

public record RawAudio (
        Path filepath,
        int format,
        int sampleRate,
        short[] data
){
}
