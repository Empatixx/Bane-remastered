package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;

public class Banner implements RoomObject{
    private final static int SCALE = 2;
    private final static int DIMENSION_WIDTH = 32;
    private final static int DIMENSION_HEIGHT = 32;
    private final Animation animation;

    private final Vector2f position;
    public Banner(){
        animation = Animation.builder()
                .setTotal(1, 4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 150)
                .texture(AssetsService.getTexture("ingame/banner.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
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
        return Camera.isInProjection(position, getWidth(), getHeight());
    }

    @Override
    public void update(double dt) {
        animation.update();
    }

    @Override
    public boolean canRemove() {
        return false;
    }
    public int getWidth(){
        return DIMENSION_WIDTH * SCALE;
    }
    public int getHeight(){
        return DIMENSION_HEIGHT * SCALE;
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
    }
}
