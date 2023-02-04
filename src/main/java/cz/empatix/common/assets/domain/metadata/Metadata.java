package cz.empatix.common.assets.domain.metadata;

import cz.empatix.common.assets.builders.VaoBuilder;
import org.lwjgl.opengl.GL30;

import java.util.List;

public record Metadata(
        int vao,
        int verticesVbo,
        int texturesVbo,
        int width,
        int height,
        int totalRows,
        int totalColumns,
        List<Row> sprites
) {
    public Metadata(final RawMetadata metadata, final int verticesVbo, final int texturesVbo) {
        this(VaoBuilder.builder()
                        .addVbo(0, verticesVbo)
                        .addVbo(1, texturesVbo)
                        .build(),
                metadata, verticesVbo, texturesVbo);
    }

    public Metadata(final int vao, final RawMetadata metadata, final int verticesVbo, final int texturesVbo) {
        this(vao, verticesVbo, texturesVbo, metadata.width(), metadata.height(), metadata.totalRows(),
                metadata.totalColumns(), metadata.rows());

    }


    public void destroyVao() {
        GL30.glDeleteBuffers(verticesVbo);
    }

    public void destroyVbo() {
        GL30.glDeleteBuffers(verticesVbo);
        GL30.glDeleteBuffers(texturesVbo);
    }
}
