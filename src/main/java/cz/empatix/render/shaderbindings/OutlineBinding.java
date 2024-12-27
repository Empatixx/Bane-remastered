package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector4f;

public record OutlineBinding(
        Vector2f texSize,
        Vector2f texStartPos,
        Vector2f texEndPos,
        Vector4f outlineColor
) implements ShaderProgramBinding {

    private final static float OUTLINE_THICKNESS = 0.01f;
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("outline");
        shader.bind();
        shader.setUniform4f("uOutlineColor", outlineColor);
        shader.setUniform1f("uOutlineThickness", OUTLINE_THICKNESS);
        shader.setUniform2f("uTexSize", texSize);
        shader.setUniform2f("uTexStartPos", texStartPos);
        shader.setUniform2f("uTexEndPos", texEndPos);
        return shader;
    }
}