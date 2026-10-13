package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.animation.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;

import cz.empatix.common.RngService;

public class Cobweb implements RoomObject {
    private final static int SCALE = 4;
    private final static int DIMENSION_WIDTH = 16;
    private final static int DIMENSION_HEIGHT = 16;
    private final Animation animation;
    private final Vector2f position;
    private boolean leftSide = false;

    public Cobweb() {
        animation = Animation.builder()
                .setTotal(2, 1)
                .addState(AnimationState.TYPE0, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.TYPE1, 1, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.TYPE2, 1, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1, true)
                .addState(AnimationState.TYPE3, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1, true)
                .texture(AssetsService.getTexture("ingame/cobweb.tga"))
                .scale(SCALE)
                .build();
        position = new Vector2f();
        animation.setState(AnimationState.TYPE0);
    }
    public void setRight(){
        leftSide = false;
        applyVariant();
    }
    public void setLeft(){
        leftSide = true;
        applyVariant();
    }
    private void applyVariant(){
        final int pick = new RngService(RngService.seedForCoordinate((int) position.x, (int) position.y)).nextInt(2);
        if (leftSide) {
            animation.setState(pick == 0 ? AnimationState.TYPE1 : AnimationState.TYPE3);
        } else {
            animation.setState(pick == 0 ? AnimationState.TYPE0 : AnimationState.TYPE2);
        }
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
        applyVariant();
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
