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
import org.lwjgl.opengl.GL30;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.FloatBuffer;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

import static org.lwjgl.opengl.GL33.*;

public class DynamicRenderer implements Renderer {
    private final static Logger LOGGER = LoggerFactory.getLogger(DynamicRenderer.class);
    private final int maxPerBatch;
    private final int vao;
    private final int vboTex;
    private final int vboPos;


    public DynamicRenderer() {
        maxPerBatch = 16;
        vboPos = VboBuilder.builder()
                .buffer(new float[maxPerBatch * 6 * 2])
                .dynamic()
                .build();
        vboTex = VboBuilder.builder()
                .buffer(new float[maxPerBatch * 6 * 2])
                .dynamic()
                .build();
        vao = VaoBuilder.builder()
                .addVbo(0, vboPos)
                .addVbo(1, vboTex)
                .build();
    }
    @Override
    public void render(final List<Drawable> drawables, final ShaderBinding shaderBinding, final int zDepth) {
        final List<RenderData> renderData = drawables.stream()
                .filter(Drawable::shouldRender)
                .map(Drawable::renderData)
                .toList();
        Lists.partition(renderData, maxPerBatch).forEach(batch -> {
            glBindTextureUnit(0, batch.get(0).textureId());
            final List<Vertex> vertices = batch.stream()
                    .flatMap(data -> data
                            .vertices()
                            .stream())
                    .toList();

            final FloatBuffer posBuffer = BufferUtils.createFloatBuffer(vertices.size() * 2);
            final FloatBuffer texCoordsBuffer = BufferUtils.createFloatBuffer(vertices.size() * 2);

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

            LOGGER.info("Rendering {} pos vertices", posBuffer.limit());
            glBindBuffer(GL_ARRAY_BUFFER, vboPos);
            glBufferSubData(GL_ARRAY_BUFFER, 0, posBuffer);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            LOGGER.info("Rendering {} texCoords vertices", texCoordsBuffer.limit());
            glBindBuffer(GL_ARRAY_BUFFER, vboTex);
            glBufferSubData(GL_ARRAY_BUFFER, 0, texCoordsBuffer);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            final Shader shader = shaderBinding.apply();

            glBindVertexArray(vao);
            glDrawArrays(GL_TRIANGLE_STRIP, 0, vertices.size());

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
