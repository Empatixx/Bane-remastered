package cz.empatix.render.renders;

import cz.empatix.common.assets.builders.VaoBuilder;
import cz.empatix.common.assets.builders.VboBuilder;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.opengl.GL11.GL_TEXTURE_2D;
import static org.lwjgl.opengl.GL13.GL_TEXTURE0;
import static org.lwjgl.opengl.GL13.glActiveTexture;
import static org.lwjgl.opengl.GL30.glBindVertexArray;

public class StaticRenderer implements Renderer {
    private final Map<Integer, Map<ShaderBinding, StaticRenderData>> staticRenderData;

    public StaticRenderer() {
        staticRenderData = new HashMap<>();
    }

    @Override
    public void render(final List<Drawable> drawables, final ShaderBinding shaderBinding, final int zDepth) {
        final StaticRenderData renderData = staticRenderData.computeIfAbsent(zDepth, HashMap::new)
                .computeIfAbsent(shaderBinding, s -> createNewStatic(drawables));
        if(renderData.size() < drawables.size()) {
            staticRenderData.get(zDepth).put(shaderBinding, renderData.resize(drawables.size()));
            final List<Integer> textures = drawables.stream()
                    .map(Drawable::renderData)
                    .map(RenderData::textureId)
                    .collect(Collectors.toSet())
                    .stream()
                    .toList();
            final Map<Integer, Integer> textureToUnit = IntStream.range(0, renderData.size())
                    .boxed()
                    .collect(Collectors.toMap(textures::get, unit -> unit));
            textureToUnit.forEach((txt, unit) -> glBindTextureUnit(unit, txt));


            final Shader shader = shaderBinding.apply();
            shader.setUniform1iv("textures", textureToUnit
                    .values()
                    .stream()
                    .mapToInt(i -> i)
                    .toArray());

            glBindVertexArray(renderData.vao());
            glDrawArrays(GL_QUADS, 0, renderData.size() * 4);
            glBindVertexArray(0);
            shader.unbind();
            glBindTexture(GL_TEXTURE_2D, 0);
        }

    }

    public StaticRenderData createNewStatic(final List<Drawable> drawables) {
        final int vboPos = VboBuilder.builder()
                .buffer(new float[drawables.size() * 4 * 2])
                .dynamic()
                .build();
        final int vboTex = VboBuilder.builder()
                .buffer(new float[drawables.size() * 4 * 2])
                .dynamic()
                .build();
        final int vboTexIndex = VboBuilder.builder()
                .buffer(new float[drawables.size() * 4])
                .dynamic()
                .build();
        final int vao = VaoBuilder.builder()
                .addVbo(0, vboPos)
                .addVbo(1, vboTex)
                .addVbo(2, vboTexIndex, 1)
                .build();
        return new StaticRenderData(vao, vboPos, vboTex, vboTexIndex, drawables.size());
    }

    private record StaticRenderData(
            int vao,
            int vboPos,
            int vboTex,
            int vboTexIndex,
            int size
    ) {
        public StaticRenderData resize(final int newSize) {
            return new StaticRenderData(vao, vboPos, vboTex, vboTexIndex, newSize);
        }
    }
    private void glBindTextureUnit(final int unit, final int texture) {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, texture);
    }
}
