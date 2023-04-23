package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector3f;

public class TextBinding implements ShaderBinding{
    private final Vector3f color;
    private final Matrix4f projection;
    public TextBinding(final Vector3f color,
                       final Matrix4f projection){
        this.color = color;
        this.projection = projection;
    }
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("text");
        shader.bind();
        shader.setUniform4x4f("projection", projection);
        shader.setUniform3f("color", color);
        return shader;
    }

    @Override
    public boolean equals(Object obj) {
        if(obj instanceof TextBinding){
            final TextBinding other = (TextBinding) obj;
            return other.color.equals(color) && other.projection.equals(projection);
        }
        return false;
    }
}
