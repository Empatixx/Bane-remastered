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
                .setTotal(2, 4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 150)
                .addState(AnimationState.DEATH, 1, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 150)
                .texture(AssetsService.getTexture("ingame/torch.tga"))
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
        final Vector2f cameraPos = Camera.getPosition();
        final Vector2f cameraDimensions = new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight());

        return position.x() + getWidth() > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + getHeight() > cameraPos.y()
                && position.y() < cameraPos.y() + cameraDimensions.y();
    }

    @Override
    public void update(double dt) {

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
}
