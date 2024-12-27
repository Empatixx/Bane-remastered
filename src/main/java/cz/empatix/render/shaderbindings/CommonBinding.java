package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;

public record CommonBinding () implements ShaderProgramBinding {
    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("common");
        shader.bind();
        shader.setUniform1i("texSampler", 0);
        return shader;
    }
    @Override
    public int hashCode() {
        return 0;
    }
    @Override
    public boolean equals(Object obj) {
        return obj instanceof CommonBinding;
    }

}
