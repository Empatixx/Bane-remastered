package cz.empatix.render.shaderbindings;

import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.ShaderProgram;

import static org.lwjgl.opengl.GL20.glUseProgram;

public interface ShaderBinding {
    ShaderProgram bind();
    default void unbind() {
        glUseProgram(0);
    }
}
