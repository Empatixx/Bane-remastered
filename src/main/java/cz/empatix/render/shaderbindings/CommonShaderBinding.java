package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;

public class CommonShaderBinding implements ShaderBinding{
    private final Matrix4f projectionMatrix;
    public CommonShaderBinding(final Matrix4f projection){
        this.projectionMatrix = projection;
    }
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("common");
        shader.bind();
        shader.setUniform4x4f("projection", projectionMatrix);
        return shader;
    }

    @Override
    public int hashCode() {
        return projectionMatrix.hashCode();
    }
    @Override
    public boolean equals(Object obj) {
        if(this == obj) return true;
        if(obj instanceof CommonShaderBinding){
            final CommonShaderBinding other = (CommonShaderBinding) obj;
            return other.projectionMatrix.equals(projectionMatrix);
        }
        return false;
    }
}
