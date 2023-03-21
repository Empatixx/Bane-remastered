package cz.empatix.assets.domain;

public record RawImage(int width,
                       int height,
                       int channels,
                       byte[] pixels) {
}

