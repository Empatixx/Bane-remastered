package cz.empatix.render.background;

import cz.empatix.common.assets.domain.TextureImage;
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

    public AnimatedBackground(final String texture, final int frames) {
        this.shader = AssetsService.getShader("shader");
        final TextureImage textureImage = AssetsService.getTexture(texture);
        this.animation = Animation.builder()
                .addAnimationState(AnimationState.STATIC, 0,frames, WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT, 100)
                .textureDimensions(frames,1)
                .texture(textureImage)
                .build();
        this.animation.setAnimationState(AnimationState.STATIC);
        this.commonBackground = new CommonBackground(texture);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(new Vector2f(0,0)),
                animation.getTextureId(),
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
