package cz.empatix.render.domain;

import cz.empatix.render.service.*;
import cz.empatix.render.service.Observer;
import org.joml.*;

import java.util.*;
import java.util.concurrent.ConcurrentHashMap;

import static org.lwjgl.opengl.GL41.*;

public class ShaderProgram implements Observer {
    private static final int NOT_FOUND = -1;
    private final Map<String, Integer> uniformLocations;
    private final int programId;
    private final List<Shader> shaders;
    public ShaderProgram(final List<Shader> shaders) {
        this.programId = glCreateProgram();
        this.uniformLocations = new ConcurrentHashMap<>();
        this.shaders = shaders;
        for (final Shader shader : shaders) {
            shader.attach(programId);
        }
        glLinkProgram(programId);
        if (glGetProgrami(programId, GL_LINK_STATUS) == GL_FALSE) {
            throw new RuntimeException("Error linking shader program: " + glGetProgramInfoLog(programId));
        }
        glValidateProgram(programId);
        if (glGetProgrami(programId, GL_VALIDATE_STATUS) == GL_FALSE) {
            throw new RuntimeException("Error validating shader program: " + glGetProgramInfoLog(programId));
        }
    }

    public void bind() {
        glUseProgram(programId);
    }

    public void unbind() {
        glUseProgram(0);
    }

    public Optional<Integer> getUniformLocation(final String name) {
        final Integer location = uniformLocations.computeIfAbsent(name, n -> glGetUniformLocation(programId, n));
        if (location == NOT_FOUND) {
            return Optional.empty();
        }
        return Optional.of(location);
    }

    public void setUniform1i(final String name, final int value) {
        getUniformLocation(name).ifPresent(location -> glProgramUniform1i(programId, location, value));
    }

    public void setUniform1f(final String name, final float value) {
        getUniformLocation(name).ifPresent(location -> glProgramUniform1f(programId, location, value));
    }

    public void setUniform2f(final String name, final Vector2f value) {
        getUniformLocation(name).ifPresent(location -> glProgramUniform2f(programId, location, value.x, value.y));
    }

    public void setUniform3f(final String name, final Vector3f value) {
        getUniformLocation(name).ifPresent(location -> glProgramUniform3f(programId, location, value.x, value.y, value.z));
    }

    public void setUniform4f(final String name, final Vector4f value) {
        getUniformLocation(name).ifPresent(location -> glProgramUniform4f(programId, location, value.x, value.y, value.z, value.w));
    }

    public void setUniform4i(final String name, final Vector4i value) {
        getUniformLocation(name).ifPresent(location -> glProgramUniform4i(programId, location, value.x, value.y, value.z, value.w));
    }

    public void setUniform4x4f(final String name, final Matrix4f value) {
        final float[] matrix = new float[16];
        value.get(matrix);
        getUniformLocation(name).ifPresent(location -> glProgramUniformMatrix4fv(programId, location, false, matrix));
    }


    public void destroy() {
        glDeleteProgram(programId);
        for (final Shader shader : shaders) {
            shader.destroy();
        }
    }

    @Override
    public void update(ObserveSubject subject, ObservePayload payload) {
        if (subject instanceof Camera camera){
            if (payload instanceof CameraObservePayload cameraObservePayload){
                switch (cameraObservePayload.change()){
                    case VIEW -> setUniform4x4f("u_viewMatrix", camera.getViewMatrix());
                    case PROJECTION -> setUniform4x4f("u_projectionMatrix", camera.getProjectionMatrix());
                    case VIEWPORT -> setUniform2f("u_resolution", new Vector2f(camera.getResolution().x(), camera.getResolution().y()));
                }
            }
        }
    }
}
