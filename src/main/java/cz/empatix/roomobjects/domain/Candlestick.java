package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.concurrent.ThreadLocalRandom;

public class Candlestick implements RoomObject{
    private final static int SCALE = 3;
    private final static int DIMENSION_WIDTH = 16 * SCALE;
    private final static int DIMENSION_HEIGHT = 16 * SCALE;
    private final Animation animation;
    private final Vector2f position;
    private final LightSource lightSource;
    public Candlestick() {
        animation = Animation.builder()
                .setTotal(2, 4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.DEACTIVATED, 1, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/candlestick.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
        lightSource = LightningService.getInstance().createLightSource(
                new Vector3f(212, 0, 0),
                30,
                2f
        );

    }

    public void destroy() {
        lightSource.destroy();
    }

    public void disableLight() {
        lightSource.destroy();
        animation.setState(AnimationState.DEACTIVATED);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
        );
    }

    public void setState(final AnimationState state) {
        animation.setState(state);
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(position,DIMENSION_WIDTH, DIMENSION_HEIGHT);
    }

    @Override
    public void update(double dt) {
        animation.update();
        lightSource.setPosition(position.x + DIMENSION_WIDTH / 2, position.y);
        lightSource.setIntensity((float) (1.25f + 0.5f * (0.5 * (Math.sin(System.currentTimeMillis() / 500.0 * Math.PI) * 0.5 + 0.5)
                        + 0.05 * Math.sin(System.currentTimeMillis() / 250.0 * Math.PI)
                        + 0.15 * Math.cos(System.currentTimeMillis() / 333.0 * Math.PI))));
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
    }

    @Override
    public int getWidth() {
        return DIMENSION_WIDTH * SCALE;
    }
    @Override
    public int getHeight() {
        return DIMENSION_HEIGHT * SCALE;
    }

    public Vector2f getPosition() {
        return position;
    }

}
