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

public class Torch implements RoomObject {
    private final static int SCALE = 2;
    private final static int DIMENSION_WIDTH = 32;
    private final static int DIMENSION_HEIGHT = 32;
    private final Animation animation;
    private final LightSource lightSource;
    private final Vector2f position;
    public Torch(){
        animation = Animation.builder()
                .setTotal(4, 4)
                .addState(AnimationState.LEFT_SIDE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 150)
                .addState(AnimationState.RIGHT_SIDE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 150,true)
                .addState(AnimationState.TOP, 1, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 150)
                .addState(AnimationState.DISABLED_LEFT_SIDE, 2, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.DISABLED_RIGHT_SIDE, 2, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1,true)
                .addState(AnimationState.DISABLED_TOP, 3, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/torch.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.LEFT_SIDE);
        lightSource = LightningService.getInstance().createLightSource(new Vector3f(230,201,114),75,1.25f);
        position = new Vector2f();
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
        );
    }
    public void disable(){
        final AnimationState state = animation.getCurrentState();
        switch (state){
            case LEFT_SIDE -> animation.setState(AnimationState.DISABLED_LEFT_SIDE);
            case RIGHT_SIDE -> animation.setState(AnimationState.DISABLED_RIGHT_SIDE);
            case TOP -> animation.setState(AnimationState.DISABLED_TOP);
        }
        lightSource.destroy();
    }
    public void setState(final AnimationState state){
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
        return Camera.isInProjection(position, DIMENSION_WIDTH * SCALE, DIMENSION_HEIGHT * SCALE);
    }

    @Override
    public void update(double dt) {
        animation.update();
        final AnimationState state = animation.getCurrentState();
        switch (state){
            case LEFT_SIDE -> {
                lightSource.setPosition(position.x() + DIMENSION_WIDTH/2f, position.y() + DIMENSION_HEIGHT/2f);
            }
            case RIGHT_SIDE -> {
                lightSource.setPosition(position.x() + DIMENSION_WIDTH, position.y() + DIMENSION_HEIGHT / 2f);
            }
            case TOP -> {
                lightSource.setPosition(position.x() + DIMENSION_WIDTH / 2,
                        position.y() + DIMENSION_HEIGHT / 2);
            }
        }
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

    @Override
    public void destroy() {
        lightSource.destroy();
    }
}
