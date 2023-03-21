package cz.empatix.render.domain;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.common.utils.BufferFactory;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.nio.FloatBuffer;
import java.util.HashMap;

import static org.lwjgl.opengl.GL20.*;

public record Shader (
        int program,
        int fragmentShaderId,
        int vertexShaderId,
        HashMap<String, Integer> locations
){
    public Shader(final int program,
                  final int fragmentShaderId,
                  final int vertexShaderId){
        this(program, fragmentShaderId, vertexShaderId, new HashMap<>());
    }
    public static Shader from(final RawShader rawShader){
        final String vertex = rawShader.vertex();
        final String fragment = rawShader.fragment();

        final int program = glCreateProgram();

        final int vertexShaderId = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertexShaderId, vertex);
        glCompileShader(vertexShaderId);
        checkForErrors(vertexShaderId);

        final int fragmentShaderId = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragmentShaderId, fragment);
        glCompileShader(fragmentShaderId);
        checkForErrors(fragmentShaderId);

        glAttachShader(program, vertexShaderId);
        glAttachShader(program, fragmentShaderId);

        glBindAttribLocation(program, 0, "vertices");
        glBindAttribLocation(program, 1, "textures");
        glBindAttribLocation(program, 2, "textureIndex");

        glLinkProgram(program);
        if (glGetProgrami(program, GL_LINK_STATUS) != 1) {
            throw new RuntimeException(glGetProgramInfoLog(program));
        }
        glValidateProgram(program);
        if (glGetProgrami(program, GL_VALIDATE_STATUS) != 1) {
            throw new RuntimeException(glGetProgramInfoLog(program));
        }
        return new Shader(program, fragmentShaderId, vertexShaderId);
    }
    private static void checkForErrors(final int id) {
        if (glGetShaderi(id, GL_COMPILE_STATUS) != 1) {
            throw new RuntimeException(glGetShaderInfoLog(id));
        }
    }

    public void destroy() {
        glDeleteShader(vertexShaderId);
        glDeleteShader(fragmentShaderId);
        glDeleteProgram(program);
    }

    public void bind() {
        glUseProgram(program);
    }

    public void unbind() {
        glUseProgram(0);
    }

    public void setUniform4x4f(final String name, final Matrix4f value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        final FloatBuffer buffer = BufferFactory.convertMatrix4f(value);
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniformMatrix4fv(location, false, buffer);
    }

    public void setUniform1iv(final String name, final int[] value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniform1iv(location, value);
    }

    public void setUniform1i(final String name, final int value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniform1i(location, value);
    }
    public void setUniform1f(final String name, final float value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniform1f(location, value);
    }
    public void setUniform2f(String name, Vector2f value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniform2f(location, value.x, value.y);
    }
    public void setUniform3f(String name, Vector3f value) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == -1) {
            throw new RuntimeException("Uniform " + name + " not found");
        }
        glUniform3f(location, value.x, value.y, value.z);
    }
}
