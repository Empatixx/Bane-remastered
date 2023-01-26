package cz.empatix.render.domain;

import cz.empatix.common.utils.BufferFactory;
import org.joml.Matrix4f;
import org.joml.Vector2f;

import java.nio.FloatBuffer;
import java.util.HashMap;

import static org.lwjgl.opengl.GL20.*;

public class Shader {
    private final int program;
    private final int fragmentShaderId;
    private final int vertexShaderId;
    private final HashMap<String, Integer> locations;

    public Shader(final String vs, final String fs) {
        program = glCreateProgram();
        locations = new HashMap<>();

        vertexShaderId = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertexShaderId, vs);
        glCompileShader(vertexShaderId);
        checkForErrors(vertexShaderId);

        fragmentShaderId = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragmentShaderId, fs);
        glCompileShader(fragmentShaderId);
        checkForErrors(fragmentShaderId);

        glAttachShader(program, vertexShaderId);
        glAttachShader(program, fragmentShaderId);

        glBindAttribLocation(program, 0, "vertices");
        glBindAttribLocation(program, 1, "textures");

        glLinkProgram(program);
        if (glGetProgrami(program, GL_LINK_STATUS) != 1) {
            throw new RuntimeException(glGetProgramInfoLog(program));
        }
        glValidateProgram(program);
        if (glGetProgrami(program, GL_VALIDATE_STATUS) != 1) {
            throw new RuntimeException(glGetProgramInfoLog(program));
        }
    }

    public int getShaderId() {
        return program;
    }

    public void destroy() {
        glDeleteShader(vertexShaderId);
        glDeleteShader(fragmentShaderId);
        glDeleteProgram(program);
    }

    private void checkForErrors(final int id) {
        if (glGetShaderi(id, GL_COMPILE_STATUS) != 1) {
            throw new RuntimeException(glGetShaderInfoLog(id));
        }
    }

    public void bind() {
        glUseProgram(program);
    }

    public void unbind() {
        glUseProgram(0);
    }

    public void setUniform(final String name, final Matrix4f value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        final FloatBuffer buffer = BufferFactory.convertMatrix4f(value);
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniformMatrix4fv(location, false, buffer);
    }

    public void setUniform(final String name, final int value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniform1i(location, value);
    }
    public void setUniform(final String name, final float value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniform1f(location, value);
    }
    public void setUniform(String name, Vector2f value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniform2f(location, value.x, value.y);
    }
}
