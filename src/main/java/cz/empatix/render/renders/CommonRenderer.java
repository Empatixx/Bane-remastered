package cz.empatix.render.renders;

import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.renderdata.CommonRenderData;
import cz.empatix.render.domain.renderdata.RenderData;

import java.util.function.Consumer;

import static org.lwjgl.opengl.GL30.*;

public class CommonRenderer implements Renderer {
    public void render(final RenderData renderData) {
        final CommonRenderData commonRenderData = (CommonRenderData) renderData;

        final Shader shader = commonRenderData.shader();
        final int vao = commonRenderData.vao();
        final int textureId = commonRenderData.textureId();
        final Consumer<Shader> prepareShaderFunc = commonRenderData.prepareShaderFunc();

        glActiveTexture(GL_TEXTURE0);
        shader.bind();
        glBindTexture(GL_TEXTURE_2D, textureId);
        glBindVertexArray(vao);

        prepareShaderFunc.accept(shader);

        glDrawArrays(GL_QUADS, 0, 4);

        glBindVertexArray(0);
        shader.unbind();
        glBindTexture(GL_TEXTURE_2D, 0);
    }

    @Override
    public boolean accepts(final RenderData renderData) {
        return renderData.type() == RenderData.RenderDataType.COMMON;
    }

}
