package cz.empatix.render.shaderbindings;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector3f;

import java.util.Objects;

public class TextBinding implements ShaderProgramBinding {
    private final Vector3f color;
    private final FontSize fontSize;
    public TextBinding(final Vector3f color,
                       final FontSize fontSize){
        this.color = color;
        this.fontSize = fontSize;
    }
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("text");
        shader.bind();
        shader.setUniform3f("color", color);
        return shader;
    }

    @Override
    public boolean equals(Object obj) {
        if(obj instanceof TextBinding){
            final TextBinding other = (TextBinding) obj;
            return other.color.equals(color) && other.fontSize.equals(fontSize);
        }
        return false;
    }

    @Override
    public int hashCode() {
        return Objects.hash(color, fontSize);
    }
}
