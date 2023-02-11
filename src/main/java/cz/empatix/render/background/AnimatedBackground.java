package cz.empatix.render.background;

import cz.empatix.common.assets.domain.ImageTexture;
import cz.empatix.common.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import org.joml.Vector2f;

public class AnimatedBackground implements Background {
    private final CommonBackground commonBackground;
    private final Animation animation;
    private final Shader shader;
    private final ImageTexture texture;

    public AnimatedBackground(final String texture, final String metadata) {
        this.shader = AssetsService.getShader("shader");
        this.texture = AssetsService.getTexture(texture);
        this.animation = Animation.builder()
                .addAnimationState(AnimationState.STATIC, 0,1, WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT, -1)
                .textureDimensions(1,1)
                .build();
        this.animation.setAnimationState(AnimationState.STATIC);
        this.commonBackground = new CommonBackground(texture);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(new Vector2f(0,0)),
                texture.textureId(),
                shader,
                this::prepareShader,
                RenderData.RenderDataType.DYNAMIC
        );
    }

    private void prepareShader(final Shader shader) {
        shader.setUniform4x4f("projection", Camera.getProjection());
    }

    @Override
    public boolean shouldRender() {
        return true;
    }

    @Override
    public int getZIndex() {
        return -1;
    }

    public void update() {
        animation.update();
    }
}
