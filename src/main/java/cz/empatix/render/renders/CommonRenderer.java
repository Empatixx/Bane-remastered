package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.function.Consumer;

import static org.lwjgl.opengl.GL30.*;

public class CommonRenderer implements Renderer {
    public void render(final Drawable drawable) {
        final RenderData renderData = drawable.renderData();

        final Shader shader = renderData.shader();
        final int vao = renderData.vao();
        final int textureId = renderData.textureId();
        final Consumer<Shader> prepareShaderFunc = renderData.prepareShaderFunc();

        shader.bind();
        glBindTexture(GL_TEXTURE_2D, textureId);
        prepareShaderFunc.accept(shader);

        glBindVertexArray(vao);
        glDrawArrays(GL_QUADS, 0, 4);
        glBindVertexArray(0);

        shader.unbind();
        glBindTexture(GL_TEXTURE_2D, 0);
    }

    @Override
    public boolean accepts(final Drawable drawable) {
        return true;
    }

}
