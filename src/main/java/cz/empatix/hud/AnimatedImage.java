package cz.empatix.hud;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonShaderBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class AnimatedImage implements HUDElement {
    private final TextureAtlas.Entry texture;
    private final Animation animation;
    private final Vector2f position;
    private final Vector2i dimensions;

    public AnimatedImage(final String textureName, final Vector2f position, int frames) {
        this(textureName, position, frames, true);
    }

    public AnimatedImage(final String textureName,
                         final Vector2f position,
                         final int frames,
                         final boolean centered) {
        texture = AssetsService.getTexture(textureName);
        this.dimensions = new Vector2i(texture.width() / frames, texture.height());
        if (centered) {
            position.sub(new Vector2f(dimensions.x(), dimensions.y()).div(2f));
        }
        this.animation = Animation.builder()
                .addAnimationState(AnimationState.STATIC, 0, frames, dimensions.x(), dimensions.y(), 100)
                .texture(texture)
                .build();
        this.animation.setAnimationState(AnimationState.STATIC);
        this.position = position;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                texture.textureId(),
                new CommonShaderBinding(Camera.getProjection()),
                RenderData.RenderDataType.STATIC
        );
    }

    @Override
    public boolean shouldRender() {
        return position.x() + dimensions.x() > Camera.getX() && position.x() < Camera.getTargetWidth() + Camera.getX() &&
                position.y() + dimensions.y() > Camera.getY() && position.y() < Camera.getTargetHeight() + Camera.getY();
    }

    @Override
    public int getZIndex() {
        return 1;
    }

    @Override
    public void update(double dt) {
        animation.update();
    }
}
