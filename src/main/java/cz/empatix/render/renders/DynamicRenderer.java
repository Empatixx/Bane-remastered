package cz.empatix.render.renders;

import com.google.common.collect.Lists;
import cz.empatix.render.builders.EboBuilder;
import cz.empatix.render.builders.VaoBuilder;
import cz.empatix.render.builders.VboBuilder;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.ShaderProgramBinding;
import org.lwjgl.BufferUtils;

import java.nio.FloatBuffer;
import java.util.List;

import static org.lwjgl.opengl.GL33.*;

public class DynamicRenderer implements Renderer {
    private final FloatBuffer vertexBuffer;
    private static final int MAX_PER_BATCH = 10000;
    private static final int VERTEX_BUFFER_SIZE = MAX_PER_BATCH * TOTAL_VERTICES_PER_QUAD * (3 + 2) * Float.BYTES;
    private final int vao;
    private final int vbo;
    private final int ebo;
    private ShaderProgramBinding currentShaderBinding;

    public DynamicRenderer() {
        vbo = VboBuilder.builder()
                .buffer(new float[MAX_PER_BATCH * TOTAL_VERTICES_PER_QUAD * (3 + 2)])
                .streaming()
                .build();
        ebo = EboBuilder.builder()
                .buffer(createElementIndices(MAX_PER_BATCH))
                .build();
        final int stride = (3 + 2) * Float.BYTES;
        vao = VaoBuilder.builder()
                .addVbo(0, vbo, 3, stride, 0)
                .addVbo(1, vbo, 2, stride, 3 * Float.BYTES)
                .addEbo(ebo)
                .build();

        vertexBuffer = BufferUtils.createFloatBuffer(MAX_PER_BATCH * TOTAL_VERTICES_PER_QUAD * (3 + 2));
    }

    @Override
    public void render(final List<DrawableData> data, final ShaderProgramBinding shaderProgramBinding) {
        final List<RenderData> renderData = data.stream()
                .map(DrawableData::renderData)
                .toList();
        if (renderData.isEmpty()) {
            return;
        }
        glBindTextureUnit(0, renderData.get(0).textureId());

        // Only bind shader if different from current
        if (currentShaderBinding != shaderProgramBinding) {
            if (currentShaderBinding != null) {
                currentShaderBinding.unbind();
            }
            shaderProgramBinding.bind();
            currentShaderBinding = shaderProgramBinding;
        }

        glBindVertexArray(vao);
        final List<List<RenderData>> batches = Lists.partition(renderData, MAX_PER_BATCH);

        for (final List<RenderData> batch : batches) {
            putVerticesToBuffer(batch);

            glBindBuffer(GL_ARRAY_BUFFER, vbo);
            glBufferData(GL_ARRAY_BUFFER, VERTEX_BUFFER_SIZE, GL_STREAM_DRAW); // orphan old buffer
            glBufferSubData(GL_ARRAY_BUFFER, 0, vertexBuffer);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            glDrawElements(GL_TRIANGLES, batch.size() * TOTAL_ELEMENT_VERTICES_PER_QUAD, GL_UNSIGNED_INT, 0);
        }
        glBindVertexArray(0);
        glBindTexture(GL_TEXTURE_2D, 0);
    }
    private void putVerticesToBuffer(final List<RenderData> batch) {
        vertexBuffer.clear();
        for (final RenderData batchRenderData : batch){
            for (final Vertex vertices : batchRenderData.vertices()){
                vertexBuffer.put(vertices.position().x())
                            .put(vertices.position().y())
                            .put(vertices.position().z())
                            .put(vertices.texCoords().x())
                            .put(vertices.texCoords().y());
            }
        }
        vertexBuffer.flip();
    }

    public void beginRendering() {
        currentShaderBinding = null;
    }

    public void endRendering() {
        if (currentShaderBinding != null) {
            currentShaderBinding.unbind();
            currentShaderBinding = null;
        }
    }

    private void glBindTextureUnit(final int unit, final int texture) {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, texture);
    }

    private int[] createElementIndices(final int count) {
        final int[] indices = new int[count * TOTAL_ELEMENT_VERTICES_PER_QUAD];
        int offset = 0;
        for (int i = 0; i < count; i++) {
            final int vertexOffset = i * 4;
            indices[offset++] = vertexOffset;
            indices[offset++] = vertexOffset + 1;
            indices[offset++] = vertexOffset + 2;
            indices[offset++] = vertexOffset;
            indices[offset++] = vertexOffset + 2;
            indices[offset++] = vertexOffset + 3;
        }
        return indices;
    }
}
