package cz.empatix.assets.domain;


import javax.annotation.Nonnull;

public record RawMetadata(
        @Nonnull String name,
        int width,
        int height,
        boolean spritesheet,
        boolean noimage,
        int totalRows,
        int totalColumns
){
}
