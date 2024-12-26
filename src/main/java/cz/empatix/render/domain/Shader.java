package cz.empatix.render.domain;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.assets.domain.ShaderType;
import cz.empatix.common.utils.BufferFactory;
import org.joml.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.FloatBuffer;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.stream.Collectors;

import static org.lwjgl.opengl.GL41.*;

public record Shader(
        int program,
        int fragmentShaderId,
        int vertexShaderId,
        Map<String, Integer> locations
) {
    private final static Logger LOGGER = LoggerFactory.getLogger(Shader.class);
    private final static int SHADER_UNIFORM_NOT_FOUND = -1;

    public Shader(final int program,
                  final int fragmentShaderId,
                  final int vertexShaderId) {
        this(program, fragmentShaderId, vertexShaderId, new HashMap<>());
    }

    public static Shader from(final List<RawShader> rawShaders) {
        final Map<ShaderType, RawShader> shaders = rawShaders.stream()
                .collect(Collectors.toMap(RawShader::type, rawShader -> rawShader));
        final String vertex = shaders.get(ShaderType.VERTEX).content();
        final String fragment = shaders.get(ShaderType.FRAGMENT).content();

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

    private Optional<Integer> getLocation(final String name) {
        final int location = locations.computeIfAbsent(name, k -> glGetUniformLocation(program, name));
        if (location == SHADER_UNIFORM_NOT_FOUND) {
            LOGGER.warn("Uniform {} not found", name);
            return Optional.empty();
        }
        return Optional.of(location);
    }

    public void setUniform4x4f(final String name, final Matrix4f value) {
        final FloatBuffer buffer = BufferFactory.convertMatrix4f(value);
        getLocation(name).ifPresent(location -> glUniformMatrix4fv(location, false, buffer));
    }

    public void setUniform1i(final String name, final int value) {
        getLocation(name).ifPresent(location -> glProgramUniform1i(program, location, value));
    }

    public void setUniform1f(final String name, final float value) {
        getLocation(name).ifPresent(location -> glProgramUniform1f(program, location, value));
    }

    public void setUniform2f(final String name, final Vector2f value) {
        getLocation(name).ifPresent(location -> glProgramUniform2f(program, location, value.x, value.y));
    }

    public void setUniform3f(final String name, final Vector3f value) {
        getLocation(name).ifPresent(location -> glProgramUniform3f(program, location, value.x, value.y, value.z));
    }

    public void setUniform4f(final String name, final Vector4f value) {
        getLocation(name).ifPresent(location -> glProgramUniform4f(program, location, value.x, value.y, value.z, value.w));
    }

    public void setUniform2i(final String name, final Vector2i value) {
        getLocation(name).ifPresent(location -> glProgramUniform2i(program, location, value.x, value.y));
    }

    public void setUniform4i(final String uConnections, final Vector4i value) {
        getLocation(uConnections).ifPresent(location -> glProgramUniform4i(program, location, value.x, value.y, value.z, value.w));
    }
}
