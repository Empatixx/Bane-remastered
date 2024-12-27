package cz.empatix.render.renders;

import com.google.common.collect.Lists;
import cz.empatix.render.builders.VaoBuilder;
import cz.empatix.render.builders.VboBuilder;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.ShaderProgramBinding;
import org.lwjgl.BufferUtils;

import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

import static org.lwjgl.opengl.GL33.*;

public class DynamicRenderer implements Renderer {
    public static AtomicInteger draws = new AtomicInteger(0);
    private final FloatBuffer vertexBuffer;
    private static final int MAX_PER_BATCH = 10000;
    private final int vao;
    private final int vbo;
    private final int ebo;

    public DynamicRenderer() {
        vbo = VboBuilder.builder()
                .buffer(new float[MAX_PER_BATCH * TOTAL_VERTICES_PER_QUAD * (3 + 2)])
                .streaming()
                .build();
        ebo = VboBuilder.builder()
                .buffer(new int[MAX_PER_BATCH * TOTAL_ELEMENT_VERTICES_PER_QUAD])
                .build();
        final int stride = (3 + 2) * Float.BYTES;
        vao = VaoBuilder.builder()
                .addVbo(0, vbo, 3, stride, 0)
                .addVbo(1, vbo, 2, stride, 3 * Float.BYTES)
                .build();

        vertexBuffer = BufferUtils.createFloatBuffer(MAX_PER_BATCH * TOTAL_VERTICES_PER_QUAD * (3 + 2));

        final IntBuffer elementIndices = BufferUtils.createIntBuffer(MAX_PER_BATCH * TOTAL_ELEMENT_VERTICES_PER_QUAD);
        elementIndices.put(createElementIndices(MAX_PER_BATCH));
        elementIndices.flip();

        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
        glBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, elementIndices);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
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

        shaderProgramBinding.bind();
        glBindVertexArray(vao);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
        final List<List<RenderData>> batches = Lists.partition(renderData, MAX_PER_BATCH);

        for (final List<RenderData> batch : batches) {
            putVerticesToBuffer(batch);

            glBindBuffer(GL_ARRAY_BUFFER, vbo);
            glBufferSubData(GL_ARRAY_BUFFER, 0, vertexBuffer);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            glDrawElements(GL_TRIANGLES, batch.size() * TOTAL_ELEMENT_VERTICES_PER_QUAD, GL_UNSIGNED_INT, 0);
            draws.incrementAndGet();
        }
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        glBindVertexArray(0);
        glBindTexture(GL_TEXTURE_2D, 0);
        shaderProgramBinding.unbind();
    }
    private void putVerticesToBuffer(final List<RenderData> batch) {
        vertexBuffer.clear();
        for (final RenderData batchRenderData : batch){
            for (final Vertex vertices : batchRenderData.vertices()){
                vertexBuffer.put(new float[]{vertices.position().x(), vertices.position().y(), vertices.position().z(), vertices.texCoords().x(), vertices.texCoords().y()});
            }
        }
        vertexBuffer.flip();
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
