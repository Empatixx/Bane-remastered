package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector2f;

public record MotionBlurBinding(
    Matrix4f projectionMatrix,
    Vector2f motionVector
) implements ShaderBinding{
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("motionBlur");
        shader.bind();
        shader.setUniform4x4f("projection", projectionMatrix);
        shader.setUniform1i("texSampler", 0);
        shader.setUniform2f("motionVector", motionVector);
        return shader;
    }
    @Override
    public int hashCode() {
        return projectionMatrix.hashCode();
    }
    @Override
    public boolean equals(Object obj) {
        if(this == obj) return true;
        if(obj instanceof final MotionBlurBinding other){
            return other.projectionMatrix.equals(projectionMatrix);
        }
        return false;
    }
}
