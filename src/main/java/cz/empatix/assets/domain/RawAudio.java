package cz.empatix.assets.domain;


import java.nio.file.Path;

public record RawAudio (
        Path filepath,
        int format,
        int sampleRate,
        short[] data
){
}
