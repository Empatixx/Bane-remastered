package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class WeaponRack implements RoomObject {
    private final static int SCALE = 3;
    private final static int DIMENSION_WIDTH = 26;
    private final static int DIMENSION_HEIGHT = 35;
    private final Animation animation;
    private final Vector2f position;

    public WeaponRack() {
        animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/swords.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
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
        return 2;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(position, getWidth(), getHeight());
    }

    @Override
    public void update(double dt) {
        animation.update();
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

    @Override
    public Vector2f getPosition() {
        return position;
    }
}
