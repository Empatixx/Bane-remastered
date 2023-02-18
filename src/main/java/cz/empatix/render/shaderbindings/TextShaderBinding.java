package cz.empatix.render.shaderbindings;

import cz.empatix.common.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector3f;

public class TextShaderBinding implements ShaderBinding{
    private final Vector3f color;
    private final Matrix4f projection;
    public TextShaderBinding(final Vector3f color,
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
    public int hashCode() {
        return "text".hashCode();
    }
    @Override
    public boolean equals(Object obj) {
        return obj instanceof TextShaderBinding;
    }
}
