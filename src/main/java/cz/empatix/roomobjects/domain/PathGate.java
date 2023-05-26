package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.Rectangle;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;

import static cz.empatix.render.domain.AnimationState.CLOSE;
import static cz.empatix.render.domain.AnimationState.OPEN;

public class PathGate implements RoomObject {
    private final static int SCALE = 1;
    private final static int DIMENSION_WIDTH = 64;
    private final static int DIMENSION_HEIGHT = 64;
    private final Animation animation;
    private final PhysicsBody physicsBody;
    public PathGate() {
        animation = Animation.builder()
                .setTotal(1, 4)
                .addState(OPEN, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75, true)
                .addState(CLOSE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75, false)
                .texture(AssetsService.getTexture("ingame/pathgate.tga"))
                .scale(SCALE)
                .build();
        animation.setState(OPEN);
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
        switch (animation.getCurrentState()) {
            case CLOSE:
                if (animation.getCurrentTextureIndex() != 3) {
                    animation.update();
                }
                break;
            case OPEN:
                if (animation.getCurrentTextureIndex() != 0) {
                    animation.update();
                }
                break;
        }
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

    public void setState(final boolean closed) {
        if (closed) {
            if(animation.getCurrentState() == CLOSE){
                return;
            }
            animation.setState(CLOSE);
        } else {
            if(animation.getCurrentState() == OPEN){
                return;
            }
            animation.setState(OPEN);
        }
    }
}
