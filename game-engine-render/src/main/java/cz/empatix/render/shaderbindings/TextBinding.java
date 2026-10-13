package cz.empatix.render.shaderbindings;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector3f;

import java.util.Objects;

public record TextBinding(
        Vector3f color,
        FontSize fontSize
) implements ShaderProgramBinding {
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("text");
        shader.bind();
        shader.setUniform3f("color", color);
        return shader;
    }
}
