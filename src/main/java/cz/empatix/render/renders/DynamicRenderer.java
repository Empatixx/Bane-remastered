package cz.empatix.render.renders;

import cz.empatix.common.assets.builders.VaoBuilder;
import cz.empatix.common.assets.builders.VboBuilder;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.Vertex;

import java.nio.FloatBuffer;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

import static org.lwjgl.opengl.GL33.*;

public class DynamicRenderer implements Renderer {
    private final int MAX_PER_BATCH = 32;
    private final int vao;
    private final int vboTex;
    private final int vboPos;
    private final int vboTexIndex;

    public DynamicRenderer() {
        vboPos = VboBuilder.builder()
                .dynamic()
                .build();
        vboTex = VboBuilder.builder()
                .dynamic()
                .build();
        vboTexIndex = VboBuilder.builder()
                .dynamic()
                .build();
        vao = VaoBuilder.builder()
                .addVbo(0, vboPos)
                .addVbo(1, vboTex)
                .addVbo(2, vboTexIndex, 1)
                .build();
    }

    @Override
    public boolean accepts(RenderData renderData) {
        return renderData.type() == RenderData.RenderDataType.DYNAMIC;
    }

    @Override
    public void render(RenderData renderData) {
        batchRender(List.of(renderData));
    }

    @Override
    public void batchRender(final List<RenderData> renderData) {
        final Shader shader = renderData.get(0).shader();
        final List<Vertex> vertices = renderData.stream()
                .flatMap(data -> data
                        .vertices()
                        .stream())
                .toList();

        final List<Integer> textures = renderData.stream()
                .map(RenderData::textureId)
                .collect(Collectors.toSet())
                .stream()
                .toList();
        final Map<Integer, Integer> textureToUnit = IntStream.range(0, textures.size())
                .boxed()
                .collect(Collectors.toMap(textures::get, unit -> unit));
        textureToUnit.forEach((txt, unit) -> glBindTextureUnit(unit, txt));

        final FloatBuffer posBuffer = FloatBuffer.allocate(vertices.size() * 2);
        final FloatBuffer texCoordsBuffer = FloatBuffer.allocate(vertices.size() * 2);
        final FloatBuffer textureUnitBuffer = FloatBuffer.allocate(renderData.size());

        vertices.stream()
                .map(Vertex::position)
                .flatMap(p -> Stream.of(p.x, p.y))
                .forEach(posBuffer::put);
        vertices.stream()
                .map(Vertex::texCoords)
                .flatMap(t -> Stream.of(t.x, t.y))
                .forEach(texCoordsBuffer::put);
        renderData.stream()
                .map(RenderData::textureId)
                .map(textureToUnit::get)
                .forEach(textureUnitBuffer::put);

        posBuffer.flip();
        texCoordsBuffer.flip();
        textureUnitBuffer.flip();

        glBindBuffer(GL_ARRAY_BUFFER, vboPos);
        glBufferSubData(GL_ARRAY_BUFFER, 0, posBuffer);
        glBindBuffer(GL_ARRAY_BUFFER, 0);

        glBindBuffer(GL_ARRAY_BUFFER, vboTex);
        glBufferSubData(GL_ARRAY_BUFFER, 0, texCoordsBuffer);
        glBindBuffer(GL_ARRAY_BUFFER, 0);

        glBindBuffer(GL_ARRAY_BUFFER, vboTexIndex);
        glBufferSubData(GL_ARRAY_BUFFER, 0, textureUnitBuffer);
        glBindBuffer(GL_ARRAY_BUFFER, 0);

        shader.bind();
        shader.setUniform("projection", Camera.getProjection());
        shader.setUniform("max_textures", 32);
        glBindTexture(GL_TEXTURE_2D, renderData.get(0).textureId());
        glBindVertexArray(vao);

        glDrawArrays(GL_QUADS, 0, vertices.size());

        glBindVertexArray(0);
        shader.unbind();
        glBindTexture(GL_TEXTURE_2D, 0);

    }

    private void glBindTextureUnit(final int unit, final int texture) {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, texture);
    }
}
