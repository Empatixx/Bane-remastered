package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector4i;

import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.opengl.GL20.glUseProgram;

public record BigMiniMapBinding(
        Matrix4f projectionMatrix,
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
    private static final int BORDER_OFFSET = 20;
    private static final int LAYOUT_SIZE_X = 400 - BORDER_OFFSET * 2;
    private static final int LAYOUT_SIZE_Y = 400 - BORDER_OFFSET * 2;
    private static final int LAYOUT_POS_X = 280 + BORDER_OFFSET;
    private static final int LAYOUT_POS_Y = 70 + BORDER_OFFSET;

    public BigMiniMapBinding(final Matrix4f projection, final Vector2f normSubImagePos, final Vector2f normSpriteSize,
                             final float time, final float randomOffset, final boolean visited,
                             final boolean current, final float alpha, final int spriteColumn,
                             final Vector4i connections) {
        this(projection, normSubImagePos, normSpriteSize, time, randomOffset, visited, current, alpha, spriteColumn, connections, time + randomOffset, normSpriteSize.x * spriteColumn);
    }

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("miniMap");
        glEnable(GL_SCISSOR_TEST);

        final float xScale = (float) WindowHandler.WIDTH / WindowHandler.TARGET_WIDTH;
        final float yScale = (float) WindowHandler.HEIGHT / WindowHandler.TARGET_HEIGHT;

        glScissor((int) (LAYOUT_POS_X * xScale), (int) (LAYOUT_POS_Y * yScale), (int) (LAYOUT_SIZE_X * xScale), (int) (LAYOUT_SIZE_Y * yScale));

        shader.bind();
        shader.setUniform4x4f("projection", projectionMatrix);
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

    @Override
    public void unbind() {
        glDisable(GL_SCISSOR_TEST);
        glUseProgram(0);
    }

}
