package cz.empatix.assets.domain.metadata;

import org.lwjgl.opengl.GL30;

import java.util.List;

import static org.lwjgl.opengl.GL11.GL_FLOAT;
import static org.lwjgl.opengl.GL15.GL_ARRAY_BUFFER;

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
        this(GL30.glGenVertexArrays(), metadata, verticesVbo, texturesVbo);
    }

    public Metadata(final int vao, final RawMetadata metadata, final int verticesVbo, final int texturesVbo) {
        this(vao, verticesVbo, texturesVbo, metadata.width(), metadata.height(), metadata.totalRows(),
                metadata.totalColumns(), metadata.rows());

        GL30.glBindVertexArray(vao);

        GL30.glBindBuffer(GL_ARRAY_BUFFER, verticesVbo);
        GL30.glVertexAttribPointer(0, 2, GL_FLOAT, false, 0, 0);

        GL30.glBindBuffer(GL_ARRAY_BUFFER, texturesVbo);
        GL30.glVertexAttribPointer(1, 2, GL_FLOAT, false, 0, 0);

        GL30.glEnableVertexAttribArray(0);
        GL30.glEnableVertexAttribArray(1);

        GL30.glBindVertexArray(0);
    }


    public void destroyVao() {
        GL30.glDeleteBuffers(verticesVbo);
    }

    public void destroyVbo() {
        GL30.glDeleteBuffers(verticesVbo);
        GL30.glDeleteBuffers(texturesVbo);
    }
}
