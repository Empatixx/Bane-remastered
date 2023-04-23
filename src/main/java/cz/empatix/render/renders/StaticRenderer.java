package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.builders.VaoBuilder;
import cz.empatix.render.builders.VboBuilder;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.shaderbindings.ShaderBinding;
import org.lwjgl.BufferUtils;

import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.*;

import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.opengl.GL13.GL_TEXTURE0;
import static org.lwjgl.opengl.GL13.glActiveTexture;
import static org.lwjgl.opengl.GL15.*;
import static org.lwjgl.opengl.GL30.glBindVertexArray;
import static org.lwjgl.opengl.GL30.glDeleteVertexArrays;

public class StaticRenderer implements Renderer {
    private final static int MAX_LIFETIME_WITHOUT_RENDER = 1000;
    private final Map<Integer, Map<Integer, StaticRenderData>> staticRenderData;

    public StaticRenderer() {
        staticRenderData = new HashMap<>();
    }

    private static final class StaticRenderData {
        private final int vao;
        private final int vbo;
        private final int ebo;
        private long lastRendered;

        private StaticRenderData(
                int vao,
                int vbo,
                int ebo

        ) {
            this.vao = vao;
            this.vbo = vbo;
            this.ebo = ebo;
            lastRendered = System.currentTimeMillis();
        }

        public static StaticRenderData from(final List<Drawable> drawables) {
            final int size = drawables.size();
            final int vbo = VboBuilder.builder()
                    .buffer(new float[size * TOTAL_VERTICES_PER_QUAD * (2 + 2)])
                    .build();
            final int ebo = VboBuilder.builder()
                    .buffer(new int[size * TOTAL_ELEMENT_VERTICES_PER_QUAD])
                    .dynamic()
                    .build();
            final int vao = VaoBuilder.builder()
                    .addVbo(0, vbo, 2, TOTAL_VERTICES_PER_QUAD * Float.BYTES, 0)
                    .addVbo(1, vbo, 2, TOTAL_VERTICES_PER_QUAD * Float.BYTES, 2 * Float.BYTES)
                    .build();

            final FloatBuffer vertexBuffer = BufferUtils.createFloatBuffer(size * TOTAL_VERTICES_PER_QUAD * (2 + 2));
            drawables
                    .stream()
                    .map(Drawable::renderData)
                    .flatMap(r -> r.vertices().stream())
                    .forEach(v -> {
                        vertexBuffer.put(v.position().x());
                        vertexBuffer.put(v.position().y());
                        vertexBuffer.put(v.texCoords().x());
                        vertexBuffer.put(v.texCoords().y());
                    });
            vertexBuffer.flip();

            glBindBuffer(GL_ARRAY_BUFFER, vbo);
            glBufferSubData(GL_ARRAY_BUFFER, 0, vertexBuffer);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            return new StaticRenderData(vao, vbo, ebo);
        }

        public void destroy() {
            glDeleteVertexArrays(vao);
            glDeleteBuffers(vbo);
            glDeleteBuffers(ebo);
        }

        public int vao() {
            return vao;
        }

        public int ebo() {
            return ebo;
        }

        public void updateLastRendered() {
            lastRendered = System.currentTimeMillis();
        }

        public boolean canDestroy() {
            return System.currentTimeMillis() - lastRendered > MAX_LIFETIME_WITHOUT_RENDER;
        }
    }

    @Override
    public void render(final List<Drawable> drawables, final ShaderBinding shaderBinding, final int zDepth) {
        if (drawables.isEmpty()) {
            return;
        }
        final StaticRenderData renderData = staticRenderData.computeIfAbsent(zDepth, i -> new HashMap<>())
                .computeIfAbsent(drawables.hashCode(), s -> StaticRenderData.from(drawables));
        render(drawables, shaderBinding, renderData);
        renderData.updateLastRendered();

        staticRenderData.forEach((z, map) -> {
                    final Iterator<Map.Entry<Integer, StaticRenderData>> iterator = map.entrySet().iterator();
                    while (iterator.hasNext()) {
                        Map.Entry<Integer, StaticRenderData> entry = iterator.next();
                        if (entry.getValue().canDestroy()) {
                            entry.getValue().destroy();
                            iterator.remove();
                        }
                    }
                }
        );
    }

    private void render(final List<Drawable> drawables,
                        final ShaderBinding shaderBinding,
                        final StaticRenderData renderData) {
        final Shader shader = shaderBinding.apply();
        glBindTextureUnit(0, drawables.get(0).renderData().textureId());

        final IntBuffer elementIndices = BufferUtils.createIntBuffer(drawables.size() * TOTAL_ELEMENT_VERTICES_PER_QUAD);
        final int[] indices = createElementIndices(drawables);

        elementIndices.put(indices);
        elementIndices.flip();

        /*glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, renderData.ebo());
        glBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, elementIndices);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);*/

        glBindVertexArray(renderData.vao());
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, renderData.ebo());

        glBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, elementIndices);

        glDrawElements(GL_TRIANGLES, indices.length, GL_UNSIGNED_INT, 0);

        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        glBindVertexArray(0);
        shader.unbind();
        glBindTexture(GL_TEXTURE_2D, 0);
    }

    private void glBindTextureUnit(final int unit, final int texture) {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, texture);
    }

    private int[] createElementIndices(final List<Drawable> drawables) {
        final int size = drawables.size();
        int[] indices = new int[size * 6];
        int currentIndex = 0;

        for (int i = 0; i < size; i++) {
            if (drawables.get(i).canRender()) {
                int base = i * 4;
                indices[currentIndex++] = base;
                indices[currentIndex++] = base + 1;
                indices[currentIndex++] = base + 2;
                indices[currentIndex++] = base;
                indices[currentIndex++] = base + 2;
                indices[currentIndex++] = base + 3;
            }
        }

        return Arrays.copyOf(indices, currentIndex);
    }
}
