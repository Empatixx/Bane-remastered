package cz.empatix.render.domain;

import static org.lwjgl.opengl.GL20.*;

public class Shader {
    private final int shaderId;
    public Shader(final String shaderCode, final int shaderType) {
        this.shaderId = glCreateShader(shaderType);
        glShaderSource(shaderId, shaderCode);
        glCompileShader(shaderId);
        if (glGetShaderi(shaderId, GL_COMPILE_STATUS) == GL_FALSE) {
            throw new RuntimeException("Error compiling shader: " + glGetShaderInfoLog(shaderId));
        }
    }
    public void attach(final int programId) {
        glAttachShader(programId, shaderId);
    }

    public void destroy() {
        glDeleteShader(shaderId);
    }
}

