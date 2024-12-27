package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;

public record CommonBinding (
    Matrix4f projectionMatrix
) implements ShaderBinding{
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("common");
        shader.bind();
        shader.setUniform4x4f("projection", projectionMatrix);
        shader.setUniform1i("texSampler", 0);
        return shader;
    }
    @Override
    public int hashCode() {
        return projectionMatrix.hashCode();
    }
    @Override
    public boolean equals(Object obj) {
        if(this == obj) return true;
        if(obj instanceof final CommonBinding other){
            return other.projectionMatrix.equals(projectionMatrix);
        }
        return false;
    }
}
