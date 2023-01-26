package cz.empatix.render;

import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.function.Consumer;

import static org.lwjgl.opengl.GL30.*;

public class Renderer {
    final Class<?> lastRenderedClass = null;
    public void render(final Drawable drawable){
        render(List.of(drawable));
    }
    public void render(final List<Drawable> drawable){
        final List<Drawable> allDrawables = new ArrayList<>(drawable);

        final List<Drawable> foundSubDrawables = new ArrayList<>();
        final List<Drawable> searchSubDrawables = new ArrayList<>(drawable);
        do{
            foundSubDrawables.clear();
            searchSubDrawables.forEach(d -> foundSubDrawables.addAll(d.getSubDrawables()));
            allDrawables.addAll(foundSubDrawables);
            searchSubDrawables.clear();
            searchSubDrawables.addAll(foundSubDrawables);
        } while(!searchSubDrawables.isEmpty());

        allDrawables.stream()
                .filter(Drawable::shouldRender)
                .sorted(Comparator.comparing(a -> a.getClass().getName()))
                .map(Drawable::renderData)
                .forEach(this::render);
    }
    public void render(final RenderData renderData){
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
}
