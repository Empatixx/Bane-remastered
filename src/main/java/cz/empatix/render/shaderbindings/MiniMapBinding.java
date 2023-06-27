package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector2f;

public class MiniMapBinding implements ShaderBinding {
    private final Matrix4f projectionMatrix;
    private final Vector2f normSubImagePos;
    private final Vector2f normSpriteSize;
    private final float normSpriteShift;
    private final boolean visited;
    private final float alpha;
    private final float time;


    public MiniMapBinding(final Matrix4f projection, final Vector2f normSubImagePos, final Vector2f normSpriteSize,
                          final float time, final boolean visited, final float alpha, final int spriteCollumn) {
        this.normSpriteSize = normSpriteSize;
        this.normSubImagePos = normSubImagePos;
        this.projectionMatrix = projection;
        this.time = time;
        this.visited = visited;
        this.alpha = alpha;
        this.normSpriteShift = spriteCollumn * normSpriteSize.x;
    }

    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("miniMap");
        shader.bind();
        shader.setUniform4x4f("projection", projectionMatrix);
        shader.setUniform1i("uTexSampler", 0);
        shader.setUniform1i("uVisited", visited ? 1 : 0);
        shader.setUniform1i("uCurrent", 1);
        shader.setUniform1f("uAlpha", alpha);
        shader.setUniform1f("uTime", time);
        shader.setUniform2f("uSpriteSize", normSubImagePos);
        shader.setUniform2f("uSubImagePos", normSpriteSize);
        shader.setUniform1f("uSpriteShift", normSpriteShift);
        return shader;
    }
}
