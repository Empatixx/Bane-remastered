package cz.empatix.background;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonShaderBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;

public class AnimatedBackground implements Background {
    private final CommonBackground commonBackground;
    private final Animation animation;

    public AnimatedBackground(final String texture, final int frames) {
        final TextureAtlas.Entry textureImage = AssetsService.getTexture(texture);
        this.animation = Animation.builder()
                .addAnimationState(AnimationState.STATIC, 0, frames, WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT, 100)
                .texture(textureImage)
                .build();
        this.animation.setAnimationState(AnimationState.STATIC);
        this.commonBackground = new CommonBackground(texture);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(new Vector2f(0, 0)),
                animation.getTextureId(),
                new CommonShaderBinding(Camera.getProjection()),
                RenderData.RenderDataType.DYNAMIC
        );
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
