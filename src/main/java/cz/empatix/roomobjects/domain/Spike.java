package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.Rectangle;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;

public class Spike implements RoomObject {
    private final static int SCALE = 4;
    private final static int DIMENSION_WIDTH = 16;
    private final static int DIMENSION_HEIGHT = 16;
    private final Animation animation;
    private final PhysicsBody physicsBody;
    public Spike(){
        animation = Animation.builder()
                .setTotal(1, 4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .texture(AssetsService.getTexture("ingame/spike.tga"))
                .scale(SCALE)
                .build();
        physicsBody = PhysicsBodyFactory.createDynamic(Rectangle.from(DIMENSION_WIDTH * SCALE, DIMENSION_HEIGHT * SCALE));
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getTranslation()),
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

    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
    }
    public float getWidth() {
        return DIMENSION_WIDTH * SCALE;
    }

    public float getHeight() {
        return DIMENSION_HEIGHT * SCALE;
    }
}
