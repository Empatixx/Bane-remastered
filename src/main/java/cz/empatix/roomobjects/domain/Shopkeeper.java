package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class Shopkeeper implements RoomObject{
    private static final int SHOPKEEPER_WIDTH = 80;
    private static final int SHOPKEEPER_HEIGHT = 80;
    private static final int SCALE = 4;
    private final Animation animation;
    private final Vector2f position;
    public Shopkeeper(){
        animation = Animation.builder()
                .setTotal(1,10)
                .addState(AnimationState.IDLE, 0, 10, SHOPKEEPER_WIDTH, SHOPKEEPER_HEIGHT, 170)
                .addState(AnimationState.REVERSE, 0, 10, SHOPKEEPER_WIDTH, SHOPKEEPER_HEIGHT, 170, false, true)
                .texture(AssetsService.getTexture("ingame/shopkeeper.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f(0,0);
    }
    public void setPosition(final float x, final float y){
        position.x = x;
        position.y = y;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
        );    }

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
        final Vector2f cameraPos = Camera.getPosition();
        final Vector2f cameraDimensions = new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight());

        return position.x() + SHOPKEEPER_WIDTH * SCALE > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + SHOPKEEPER_HEIGHT * SCALE > cameraPos.y()
                && position.y() < cameraPos.y() + cameraDimensions.y();
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
        return SHOPKEEPER_WIDTH * SCALE;
    }
    public int getHeight(){
        return SHOPKEEPER_HEIGHT * SCALE;
    }
}
