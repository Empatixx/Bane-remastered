package cz.empatix.render.renders;

import cz.empatix.assets.builders.VaoBuilder;
import cz.empatix.assets.builders.VboBuilder;
import cz.empatix.common.utils.BufferFactory;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.ShaderBinding;
import org.lwjgl.BufferUtils;

import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.opengl.GL11.GL_QUADS;
import static org.lwjgl.opengl.GL13.GL_TEXTURE0;
import static org.lwjgl.opengl.GL13.glActiveTexture;
import static org.lwjgl.opengl.GL15.*;
import static org.lwjgl.opengl.GL30.glBindVertexArray;
import static org.lwjgl.opengl.GL30.glDeleteVertexArrays;

public class StaticRenderer implements Renderer {
    private final Map<Integer, Map<ShaderBinding, StaticRenderData>> staticRenderData;
    public StaticRenderer() {
        staticRenderData = new HashMap<>();
    }
    private record StaticRenderData(
            int vao,
            int vboPos,
            int vboTex,
            int ebo,
            int hash

    ) {
        public static StaticRenderData from(final List<Drawable> drawables){
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

            return new StaticRenderData(vao, vboPos, vboTex, ebo, drawables.hashCode());
        }
        public void destroy(){
            glDeleteVertexArrays(vao);
            glDeleteBuffers(vboPos);
            glDeleteBuffers(vboTex);
            glDeleteBuffers(ebo);
        }
    }

    @Override
    public void render(final List<Drawable> drawables, final ShaderBinding shaderBinding, final int zDepth) {
        final StaticRenderData renderData = staticRenderData.computeIfAbsent(zDepth, HashMap::new)
                .computeIfAbsent(shaderBinding, s -> StaticRenderData.from(drawables));

        if(renderData.hash() != drawables.hashCode()){
            final StaticRenderData newRenderData = StaticRenderData.from(drawables);
            staticRenderData.get(zDepth).replace(shaderBinding, newRenderData);
            renderData.destroy();
            render(drawables, shaderBinding, newRenderData);
            return;
        }
        render(drawables, shaderBinding, renderData);
    }
    private void render(final List<Drawable> drawables,
                        final ShaderBinding shaderBinding,
                        final StaticRenderData renderData){
        final Shader shader = shaderBinding.apply();
        glBindTextureUnit(0, drawables.get(0).renderData().textureId());

        final IntBuffer elementIndices = BufferUtils.createIntBuffer(drawables.size() * 6);

        elementIndices.put(createElementIndices(drawables));
        elementIndices.flip();

        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, renderData.ebo());
        glBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, elementIndices);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);

        glBindVertexArray(renderData.vao());
        glDrawArrays(GL_QUADS, 0, drawables * 4);

        glBindVertexArray(0);
        shader.unbind();
        glBindTexture(GL_TEXTURE_2D, 0);
    }
    private void glBindTextureUnit(final int unit, final int texture) {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, texture);
    }
    private int[] createElementIndices(final int count) {
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
