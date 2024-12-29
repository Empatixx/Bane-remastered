package cz.empatix.artefacts.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;

public class MagicShield implements Drawable {
    private static final int SCALE = 3;
    private static final int DIMENSION_WIDTH = 32 * SCALE;
    private static final int DIMENSION_HEIGHT = 48 * SCALE;
    private final Animation animation;
    private final Vector2f position;
    public MagicShield() {
        position = new Vector2f();
        animation = Animation.builder()
                .setTotal(1, 4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/artefacts/magicshield.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
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
        return 4;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    public void setPosition(float x, float y) {
        position.set(x, y);
    }
    public void update(final double dt) {
        animation.update();
    }

    public int getWidth() {
        return DIMENSION_WIDTH;
    }

    public int getHeight() {
        return DIMENSION_HEIGHT;
    }
}
