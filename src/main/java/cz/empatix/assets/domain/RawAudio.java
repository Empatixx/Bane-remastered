package cz.empatix.assets.domain;


public record RawAudio (
        int format,
        int sampleRate,
        short[] data
){
}
