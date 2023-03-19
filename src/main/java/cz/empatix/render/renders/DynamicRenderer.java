package cz.empatix.render.renders;

import com.google.common.collect.Lists;
import cz.empatix.assets.builders.VaoBuilder;
import cz.empatix.assets.builders.VboBuilder;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.ShaderBinding;
import org.lwjgl.BufferUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.List;
import java.util.stream.Stream;

import static org.lwjgl.opengl.GL33.*;

public class DynamicRenderer implements Renderer {
    private final static Logger LOGGER = LoggerFactory.getLogger(DynamicRenderer.class);
    private final FloatBuffer posBuffer;
    private final FloatBuffer texCoordsBuffer;
    private final int maxPerBatch;
    private final int vao;
    private final int vboTex;
    private final int vboPos;
    private final int ebo;


    public DynamicRenderer() {
        maxPerBatch = 255;
        vboPos = VboBuilder.builder()
                .buffer(new float[maxPerBatch * 4 * 2])
                .dynamic()
                .build();
        vboTex = VboBuilder.builder()
                .buffer(new float[maxPerBatch * 4 * 2])
                .dynamic()
                .build();
        ebo = VboBuilder.builder()
                .buffer(new int[maxPerBatch * 6])
                .build();
        vao = VaoBuilder.builder()
                .addVbo(0, vboPos)
                .addVbo(1, vboTex)
                .build();

        posBuffer = BufferUtils.createFloatBuffer(maxPerBatch * 4 * 2);
        texCoordsBuffer = BufferUtils.createFloatBuffer(maxPerBatch * 4 * 2);

        final IntBuffer elementIndices = BufferUtils.createIntBuffer(maxPerBatch * 6);
        elementIndices.put(createIndices(maxPerBatch));
        elementIndices.flip();

        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
        glBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, elementIndices);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    }
    @Override
    public void render(final List<Drawable> drawables, final ShaderBinding shaderBinding, final int zDepth) {
        final List<RenderData> renderData = drawables.stream()
                .filter(Drawable::shouldRender)
                .map(Drawable::renderData)
                .toList();
        glBindTextureUnit(0, renderData.get(0).textureId());
        final Shader shader = shaderBinding.apply();

        Lists.partition(renderData, maxPerBatch).forEach(batch -> {
            posBuffer.clear();
            texCoordsBuffer.clear();
            final List<Vertex> vertices = batch.stream()
                    .flatMap(data -> data
                            .vertices()
                            .stream())
                    .toList();
            vertices.stream()
                    .map(Vertex::position)
                    .flatMap(p -> Stream.of(p.x, p.y))
                    .forEach(posBuffer::put);
            vertices.stream()
                    .map(Vertex::texCoords)
                    .flatMap(t -> Stream.of(t.x, t.y))
                    .forEach(texCoordsBuffer::put);
            posBuffer.flip();
            texCoordsBuffer.flip();

            glBindBuffer(GL_ARRAY_BUFFER, vboPos);
            glBufferSubData(GL_ARRAY_BUFFER, 0, posBuffer);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            glBindBuffer(GL_ARRAY_BUFFER, vboTex);
            glBufferSubData(GL_ARRAY_BUFFER, 0, texCoordsBuffer);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            glBindVertexArray(vao);
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

            glDrawElements(GL_TRIANGLES, batch.size() * 6, GL_UNSIGNED_INT, 0);

            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
            glBindVertexArray(0);
        });
        glBindTexture(GL_TEXTURE_2D, 0);
        shader.unbind();
    }

    private void glBindTextureUnit(final int unit, final int texture) {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, texture);
    }
    private int[] createIndices(final int count) {
        final int[] indices = new int[count * 6];
        for (int i = 0; i < count; i++) {
            final int offset = i * 6;
            final int vertexOffset = i * 4;
            indices[offset] = vertexOffset;
            indices[offset + 1] = vertexOffset + 1;
            indices[offset + 2] = vertexOffset + 2;
            indices[offset + 3] = vertexOffset;
            indices[offset + 4] = vertexOffset + 2;
            indices[offset + 5] = vertexOffset + 3;
        }
        return indices;
    }
}
