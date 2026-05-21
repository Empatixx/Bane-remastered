package cz.empatix.shaderbindings;

import cz.empatix.render.shaderbindings.ShaderProgramBinding;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector4i;

public record MiniMapBinding(
        Vector2f normSubImagePos,
        Vector2f normSpriteSize,
        float time,
        float randomOffset,
        boolean visited,
        @Deprecated boolean current,
        float alpha,
        int spriteColumn,
        Vector4i connections,
        float timeWithOffset,
        float normSpriteShift
) implements ShaderProgramBinding {

    public MiniMapBinding(final Vector2f normSubImagePos, final Vector2f normSpriteSize,
                          final float time, final float randomOffset, final boolean visited,
                          final boolean current, final float alpha, final int spriteColumn,
                          final Vector4i connections) {
        this(normSubImagePos, normSpriteSize, time, randomOffset, visited, current, alpha,  spriteColumn, connections, time+randomOffset, normSpriteSize.x * spriteColumn);
    }

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("miniMap");
        shader.bind();
        shader.setUniform1i("uTexSampler", 0);
        shader.setUniform1i("uVisited", visited ? 1 : 0);
        shader.setUniform1f("uAlpha", alpha);
        shader.setUniform1f("uTime", time);
        shader.setUniform1f("uTimeWithOffset", timeWithOffset);
        shader.setUniform4i("uConnections", connections);
        shader.setUniform2f("uSpriteSize", normSpriteSize);
        shader.setUniform2f("uSubImagePos", normSubImagePos);
        shader.setUniform1f("uSpriteShift", normSpriteShift);
        return shader;
    }
}
