package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector4f;

public record OutlineBinding(
        Matrix4f projection,
        Vector2f texSize,
        Vector2f texStartPos,
        Vector2f texEndPos,
        Vector4f outlineColor

) implements ShaderBinding{

    private final static float OUTLINE_THICKNESS = 0.01f;
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("outline");
        shader.bind();
        shader.setUniform4f("uOutlineColor", outlineColor);
        shader.setUniform1f("uOutlineThickness", OUTLINE_THICKNESS);
        shader.setUniform2f("uTexSize", texSize);
        shader.setUniform4x4f("uProjection", projection);
        shader.setUniform2f("uTexStartPos", texStartPos);
        shader.setUniform2f("uTexEndPos", texEndPos);
        return shader;
    }
}