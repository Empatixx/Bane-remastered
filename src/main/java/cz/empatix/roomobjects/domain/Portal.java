package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class Portal implements RoomObject {
    private final static int SCALE = 2;
    private final static int DIMENSION_WIDTH = 96;
    private final static int DIMENSION_HEIGHT = 90;
    private final Animation animation;
    private final Vector2f position;

    public Portal(final Vector2f position) {
        animation = Animation.builder()
                .setTotal(1, 8)
                .addState(AnimationState.IDLE, 0, 8, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                .texture(AssetsService.getTexture("ingame/portal.tga"))
                .scale(SCALE)
                .build();
        this.position = position;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
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
        return true;
    }

    @Override
    public void update(double dt) {
        animation.update();
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    public float getWidth() {
        return DIMENSION_WIDTH * SCALE;
    }

    public float getHeight() {
        return DIMENSION_HEIGHT * SCALE;
    }
}
