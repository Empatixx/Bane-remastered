package cz.empatix.render.shaderbindings;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Vector3f;

import java.util.Objects;

public class HudTextBinding implements ShaderProgramBinding {
    private final Vector3f color;
    private final FontSize fontSize;
    public HudTextBinding(final Vector3f color,
                       final FontSize fontSize){
        this.color = color;
        this.fontSize = fontSize;
    }
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("hudText");
        shader.bind();
        shader.setUniform3f("color", color);
        return shader;
    }

}