package cz.empatix.render;

import cz.empatix.assets.domain.Metadata;
import cz.empatix.assets.service.AssetsService;

import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.opengl.GL11.GL_QUADS;
import static org.lwjgl.opengl.GL15.GL_ARRAY_BUFFER;
import static org.lwjgl.opengl.GL15.glBindBuffer;
import static org.lwjgl.opengl.GL20.*;
import static org.lwjgl.opengl.GL20.glDisableVertexAttribArray;

public class Background implements Drawable {
    private final Shader shader;
    private final int texture;
    private final int verticesVbo;
    private final int texturesVbo;

    public Background(final String texture) {
        this.shader = AssetsService.getShader("shader");
        this.texture = AssetsService.getTexture(texture);
        final Metadata metadata = AssetsService.getMetadata("background-overlay.json");
        texturesVbo = metadata.getTexturesVbos(0).get(0);
        verticesVbo = metadata.verticesVbo();
    }

    @Override
    public void draw() {
        shader.bind();
        shader.setUniform("sampler", 0);
        shader.setUniform("projection", Camera.getProjection());

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture);

        glEnableVertexAttribArray(0);
        glEnableVertexAttribArray(1);

        glBindBuffer(GL_ARRAY_BUFFER, verticesVbo);
        glVertexAttribPointer(0, 2, GL_FLOAT, false, 0, 0);

        glBindBuffer(GL_ARRAY_BUFFER, texturesVbo);
        glVertexAttribPointer(1, 2, GL_FLOAT, false, 0, 0);

        glDrawArrays(GL_QUADS, 0, 4);

        glBindBuffer(GL_ARRAY_BUFFER, 0);

        glDisableVertexAttribArray(0);
        glDisableVertexAttribArray(1);
        shader.unbind();
        glBindTexture(GL_TEXTURE_2D, 0);
    }

    @Override
    public boolean shouldDraw() {
        return true;
    }
}
