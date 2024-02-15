package cz.empatix.render.shaderbindings;

import cz.empatix.render.domain.Shader;

import static org.lwjgl.opengl.GL20.glUseProgram;

public interface ShaderBinding {
    Shader apply();
    default void unbind() {
        glUseProgram(0);
    }
}
