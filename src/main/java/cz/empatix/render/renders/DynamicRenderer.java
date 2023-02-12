package cz.empatix.render.renders;

import com.google.common.collect.Lists;
import cz.empatix.common.assets.builders.VaoBuilder;
import cz.empatix.common.assets.builders.VboBuilder;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.Vertex;
import org.lwjgl.BufferUtils;

import java.nio.FloatBuffer;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

import static org.lwjgl.opengl.GL33.*;

public class DynamicRenderer implements Renderer {
    private final int maxPerBatch;
    private final int vao;
    private final int vboTex;
    private final int vboPos;
    private final int vboTexIndex;

    public DynamicRenderer() {
        maxPerBatch = 32;

        vboPos = VboBuilder.builder()
                .buffer(new float[maxPerBatch * 4 * 2])
                .dynamic()
                .build();
        vboTex = VboBuilder.builder()
                .buffer(new float[maxPerBatch * 4 * 2])
                .dynamic()
                .build();
        vboTexIndex = VboBuilder.builder()
                .buffer(new float[maxPerBatch * 4])
                .dynamic()
                .build();
        vao = VaoBuilder.builder()
                .addVbo(0, vboPos)
                .addVbo(1, vboTex)
                .addVbo(2, vboTexIndex, 1)
                .build();
    }

    @Override
    public void render(RenderData renderData) {
        batchRender(List.of(renderData));
    }

    @Override
    public void batchRender(final List<RenderData> renderData) {
        Lists.partition(renderData, maxPerBatch).forEach(batch -> {
            final List<Integer> textures = batch.stream()
                    .map(RenderData::textureId)
                    .collect(Collectors.toSet())
                    .stream()
                    .toList();
            final Map<Integer, Integer> textureToUnit = IntStream.range(0, textures.size())
                    .boxed()
                    .collect(Collectors.toMap(textures::get, unit -> unit));
            textureToUnit.forEach((txt, unit) -> glBindTextureUnit(unit, txt));

            final List<Vertex> vertices = batch.stream()
                    .flatMap(data -> data
                            .vertices()
                            .stream())
                    .toList();

            final FloatBuffer posBuffer = BufferUtils.createFloatBuffer(vertices.size() * 2);
            final FloatBuffer texCoordsBuffer = BufferUtils.createFloatBuffer(vertices.size() * 2);
            final FloatBuffer textureUnitBuffer = BufferUtils.createFloatBuffer(vertices.size());

            vertices.stream()
                    .map(Vertex::position)
                    .flatMap(p -> Stream.of(p.x, p.y))
                    .forEach(posBuffer::put);
            vertices.stream()
                    .map(Vertex::texCoords)
                    .flatMap(t -> Stream.of(t.x, t.y))
                    .forEach(texCoordsBuffer::put);
            batch.stream()
                    .flatMap(rd -> {
                        final List<Vertex> v = rd.vertices();
                        return Collections.nCopies(v.size(), rd.textureId()).stream();
                    })
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

            final Shader shader = renderData.get(0).shaderBinding().apply();
            shader.setUniform1iv("textures", textureToUnit
                    .values()
                    .stream()
                    .mapToInt(i -> i)
                    .toArray());
            glBindVertexArray(vao);

            glDrawArrays(GL_QUADS, 0, vertices.size());

            glBindVertexArray(0);
            shader.unbind();
            glBindTexture(GL_TEXTURE_2D, 0);
        });
    }

    private void glBindTextureUnit(final int unit, final int texture) {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, texture);
    }
}
