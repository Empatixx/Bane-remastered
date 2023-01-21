package cz.empatix.render;

import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;

import java.util.Comparator;
import java.util.List;
import java.util.function.Consumer;

import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.opengl.GL11.GL_TEXTURE_2D;
import static org.lwjgl.opengl.GL13.GL_TEXTURE0;
import static org.lwjgl.opengl.GL13.glActiveTexture;
import static org.lwjgl.opengl.GL15.GL_ARRAY_BUFFER;
import static org.lwjgl.opengl.GL15.glBindBuffer;
import static org.lwjgl.opengl.GL20.*;
import static org.lwjgl.opengl.GL20.glDisableVertexAttribArray;

public class Renderer {
    public void render(final Drawable drawable){
        render(List.of(drawable));
    }
    public void render(final List<Drawable> drawable){
        drawable.stream()
                .filter(Drawable::shouldRender)
                .sorted(Comparator.comparing(a -> a.getClass().getName()))
                .map(Drawable::renderData)
                .forEach(this::render);
    }
    public void render(final RenderData renderData){
        final Shader shader = renderData.shader();
        final int verticesVbo = renderData.verticesVbo();
        final int textureVbo = renderData.textureVbo();
        final int textureId = renderData.textureId();
        final Consumer<Shader> prepareShaderFunc = renderData.prepareShaderFunc();

        shader.bind();
        prepareShaderFunc.accept(shader);

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, textureId);

        glEnableVertexAttribArray(0);
        glEnableVertexAttribArray(1);

        glBindBuffer(GL_ARRAY_BUFFER, verticesVbo);
        glVertexAttribPointer(0, 2, GL_FLOAT, false, 0, 0);

        glBindBuffer(GL_ARRAY_BUFFER, textureVbo);
        glVertexAttribPointer(1, 2, GL_FLOAT, false, 0, 0);

        glDrawArrays(GL_QUADS, 0, 4);

        glBindBuffer(GL_ARRAY_BUFFER, 0);

        glDisableVertexAttribArray(0);
        glDisableVertexAttribArray(1);

        shader.unbind();
        glBindTexture(GL_TEXTURE_2D, 0);
    }
}
