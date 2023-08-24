package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector4f;

public record EnemyBinding(
        Matrix4f projection,
        Vector2f texSize,
        Vector2f texStartPos,
        Vector2f texEndPos

) implements ShaderBinding{
    private final static Vector4f COLOR = new Vector4f(1.0f, 0.0f, 0.0f,1.f);
    private final static float OUTLINE_THICKNESS = 0.1f;
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("enemy");
        shader.bind();
        shader.setUniform4f("uOutlineColor", COLOR);
        shader.setUniform1f("uOutlineThickness", OUTLINE_THICKNESS);
        shader.setUniform2f("uTexSize", texSize);
        shader.setUniform4x4f("uProjection", projection);
        shader.setUniform2f("uTexStartPos", texStartPos);
        shader.setUniform2f("uTexEndPos", texEndPos);
        shader.setUniform1i("uTexSampler", 0);
        return shader;
    }
    @Override
    public int hashCode() {
        return projection.hashCode() + texSize.hashCode() + texStartPos.hashCode() + texEndPos.hashCode();
    }
    @Override
    public boolean equals(Object obj) {
        if(this == obj) return true;
        if(obj instanceof final EnemyBinding other){
            return other.projection.equals(projection) && other.texSize.equals(texSize) && other.texStartPos.equals(texStartPos) && other.texEndPos.equals(texEndPos);
        }
        return false;
    }
}