package cz.empatix.render.shaderbindings;

import cz.empatix.render.domain.ShaderProgram;

import static org.lwjgl.opengl.GL20.glUseProgram;

public interface ShaderProgramBinding {
    ShaderProgram bind();
    default void unbind() {
        glUseProgram(0);
    }
}
