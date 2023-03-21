package cz.empatix.render.renders;

import cz.empatix.assets.builders.VaoBuilder;
import cz.empatix.assets.builders.VboBuilder;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.ShaderBinding;
import org.lwjgl.BufferUtils;

import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.*;
import java.util.stream.IntStream;
import java.util.stream.Stream;

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
        private final int vboPos;
        private final int vboTex;
        private final int ebo;
        private long lastRendered;

        private StaticRenderData(
                int vao,
                int vboPos,
                int vboTex,
                int ebo

        ) {
            this.vao = vao;
            this.vboPos = vboPos;
            this.vboTex = vboTex;
            this.ebo = ebo;
            lastRendered = System.currentTimeMillis();
        }

        public static StaticRenderData from(final List<Drawable> drawables) {
            final int size = drawables.size();
            final int vboPos = VboBuilder.builder()
                    .buffer(new float[size * 4 * 2])
                    .build();
            final int vboTex = VboBuilder.builder()
                    .buffer(new float[size * 4 * 2])
                    .build();
            final int vao = VaoBuilder.builder()
                    .addVbo(0, vboPos)
                    .addVbo(1, vboTex)
                    .build();
            final int ebo = VboBuilder.builder()
                    .buffer(new int[size * 6])
                    .build();

            final List<Vertex> vertices = drawables.stream()
                    .map(Drawable::renderData)
                    .map(RenderData::vertices)
                    .flatMap(List::stream)
                    .toList();

            final FloatBuffer posBuffer = BufferUtils.createFloatBuffer(size * 4 * 2);
            final FloatBuffer texCoordsBuffer = BufferUtils.createFloatBuffer(size * 4 * 2);

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

            return new StaticRenderData(vao, vboPos, vboTex, ebo);
        }

        public void destroy() {
            glDeleteVertexArrays(vao);
            glDeleteBuffers(vboPos);
            glDeleteBuffers(vboTex);
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
        public boolean shouldDestroy() {
            return System.currentTimeMillis() - lastRendered > MAX_LIFETIME_WITHOUT_RENDER;
        }
    }

    @Override
    public void render(final List<Drawable> drawables, final ShaderBinding shaderBinding, final int zDepth) {
        final StaticRenderData renderData = staticRenderData.computeIfAbsent(zDepth, HashMap::new)
                .computeIfAbsent(drawables.hashCode(), s -> StaticRenderData.from(drawables));
        render(drawables, shaderBinding, renderData);
        renderData.updateLastRendered();

        staticRenderData.entrySet()
                .stream()
                .filter(e -> e.getKey() != zDepth)
                .flatMap(e -> e.getValue().entrySet().stream())
                .filter(e -> e.getValue().shouldDestroy())
                .forEach(e -> e.getValue().destroy());
    }

    private void render(final List<Drawable> drawables,
                        final ShaderBinding shaderBinding,
                        final StaticRenderData renderData) {
        final Shader shader = shaderBinding.apply();
        glBindTextureUnit(0, drawables.get(0).renderData().textureId());

        final IntBuffer elementIndices = BufferUtils.createIntBuffer(drawables.size() * 6);
        final int[] indices = createElementIndices(drawables);

        elementIndices.put(indices);
        elementIndices.flip();

        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, renderData.ebo());
        glBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, elementIndices);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);

        glBindVertexArray(renderData.vao());
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, renderData.ebo());

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
        return IntStream.range(0, drawables.size())
                .filter(i -> drawables.get(i).shouldRender())
                .mapToObj(i -> {
                    int[] indices = new int[6];
                    indices[0] = i * 4;
                    indices[1] = i * 4 + 1;
                    indices[2] = i * 4 + 2;
                    indices[3] = i * 4;
                    indices[4] = i * 4 + 2;
                    indices[5] = i * 4 + 3;
                    return indices;
                })
                .flatMapToInt(Arrays::stream)
                .toArray();
    }
}
