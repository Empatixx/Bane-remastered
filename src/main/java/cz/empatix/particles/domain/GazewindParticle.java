package cz.empatix.particles.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.joml.Vector2i;

public record GazewindParticle(
        Vector2f transform,
        Animation animation
) implements Drawable {
    private final static int SCALE = 2;
    private final static Vector2i DIMENSIONS = new Vector2i(16,16).mul(SCALE);

    public GazewindParticle(final String texturePath) {
        this(new Vector2f(), Animation.builder()
                .setTotal(1, 3)
                .addState(AnimationState.IDLE, 0, 3, DIMENSIONS.x(), DIMENSIONS.y(), 100)
                .texture(AssetsService.getTexture(texturePath))
                .build());
    }

    public void update(final float dt) {
        animation.update();
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(transform, getZIndex()),
                animation.getTextureId(),
                new CommonBinding()
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 1;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(transform, DIMENSIONS.x(), DIMENSIONS.y());
    }

    public boolean shouldRemove() {
        return animation.hasPlayedOnce();
    }

    public void setPosition(final float x, final float y) {
        transform.set(x, y);
    }
    public int getWidth(){
        return DIMENSIONS.x();
    }
    public int getHeight(){
        return DIMENSIONS.y();
    }
}
