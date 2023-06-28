package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector4i;

public class MiniMapBinding implements ShaderBinding {
    private final Matrix4f projectionMatrix;
    private final Vector2f normSubImagePos;
    private final Vector2f normSpriteSize;
    private final Vector4i connections;
    private final float normSpriteShift;
    private final boolean visited;
    private final boolean current;
    private final float alpha;
    private final float time;
    private final float timeWithOffset;


    public MiniMapBinding(final Matrix4f projection, final Vector2f normSubImagePos, final Vector2f normSpriteSize,
                          final float time, final float randomOffset, final boolean visited,
                          final boolean current, final float alpha, final int spriteColumn,
                          final Vector4i connections) {
        this.normSpriteSize = normSpriteSize;
        this.normSubImagePos = normSubImagePos;
        this.projectionMatrix = projection;
        this.time = time;
        this.visited = visited;
        this.current = current;
        this.timeWithOffset = time + randomOffset;
        this.alpha = alpha;
        this.connections = connections;
        this.normSpriteShift = normSpriteSize.x * spriteColumn;
    }

    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("miniMap");
        shader.bind();
        shader.setUniform4x4f("projection", projectionMatrix);
        shader.setUniform1i("uTexSampler", 0);
        shader.setUniform1i("uVisited", visited ? 1 : 0);
        shader.setUniform1i("uCurrent", current ? 1 : 0);
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
