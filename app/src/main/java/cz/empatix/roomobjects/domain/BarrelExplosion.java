package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class BarrelExplosion implements Drawable {
    private static final int SCALE = 2;
    private static final int WIDTH = 64 * SCALE;
    private static final int HEIGHT = 64 * SCALE;
    private final Animation animation;
    private final Vector2f position;

    public BarrelExplosion() {
        animation = Animation.builder()
                .setTotal(1, 9)
                .addState(AnimationState.IDLE, 0, 9, WIDTH, HEIGHT, 80)
                .texture(AssetsService.getTexture("ingame/barrel_explosion.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 3;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(position, WIDTH, HEIGHT);
    }

    public boolean hasPlayedOnce() {
        return animation.hasPlayedOnce();
    }

    public void update(final double dt) {
        animation.update();
    }

    public void setPosition(final float x, final float y) {
        this.position.set(x, y);
    }

    public int getWidth() {
        return WIDTH;
    }

    public int getHeight() {
        return HEIGHT;
    }
}
