package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class ShopTable implements RoomObject {
    private static final int TABLE_WIDTH = 96;
    private static final int TABLE_HEIGHT = 64;
    private static final float SCALE = 1.5f;
    private final Animation animation;
    private final Vector2f position;
    public ShopTable(){
        animation = Animation.builder()
                .setTotal(1,1)
                .addState(AnimationState.IDLE, 0, 1, TABLE_WIDTH, TABLE_HEIGHT, Integer.MAX_VALUE)
                .texture(AssetsService.getTexture("ingame/shoptable.tga"))
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

        return position.x() + TABLE_WIDTH * SCALE > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + TABLE_HEIGHT * SCALE > cameraPos.y()
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
        return (int)(TABLE_WIDTH * SCALE);
    }
    public int getHeight(){
        return (int)(TABLE_HEIGHT * SCALE);
    }
    public float getX(){
        return position.x();
    }
    public float getY(){
        return position.y();
    }
}