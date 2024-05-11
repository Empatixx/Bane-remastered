package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

import static cz.empatix.render.domain.AnimationState.*;

public class PathGate implements RoomObject {
    private final static int SCALE = 2;
    private final static int DIMENSION_WIDTH = 32;
    private final static int DIMENSION_HEIGHT = 32;
    private final Animation animation;
    private final PhysicsObject physicsBody;
    private PathGateDirection direction = PathGateDirection.NORMAL;

    public PathGate() {
        animation = Animation.builder()
                .setTotal(4, 6)
                .addState(OPEN, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75, false, true)
                .addState(CLOSE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75)
                .addState(IDLE, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75)
                .addState(RIGHT_SIDE_OPEN, 2, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75, false, true)
                .addState(RIGHT_SIDE_CLOSE, 2, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75)
                .addState(RIGHT_SIDE_IDLE, 3, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75)
                .addState(LEFT_SIDE_OPEN, 2, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75, true, true)
                .addState(LEFT_SIDE_CLOSE, 2, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75, true)
                .addState(LEFT_SIDE_IDLE, 3, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75, true)
                .texture(AssetsService.getTexture("ingame/pathgate.tga"))
                .scale(SCALE)
                .build();
        animation.setState(OPEN);

        physicsBody = PhysicsObjectFactory.create(DIMENSION_WIDTH * SCALE, DIMENSION_HEIGHT * SCALE);
        physicsBody.setType(CollisionType.TRAP);
        physicsBody.setUserData(this);
    }

    public void setDirection(final PathGateDirection direction) {
        this.direction = direction;
        switch (direction) {
            case NORMAL -> animation.setState(OPEN);
            case RIGHT_SIDE -> animation.setState(RIGHT_SIDE_OPEN);
            case LEFT_SIDE -> animation.setState(LEFT_SIDE_OPEN);
        }
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getPhysicsTranslation()),
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
        return Camera.isInProjection(physicsBody.getPhysicsTranslation(), DIMENSION_WIDTH * SCALE, DIMENSION_HEIGHT * SCALE);
    }

    @Override
    public void update(double dt) {
        switch (animation.getCurrentState()) {
            case CLOSE, RIGHT_SIDE_CLOSE, LEFT_SIDE_CLOSE -> {
                animation.update();
                if (animation.hasPlayedOnce()){
                    switch (animation.getCurrentState()){
                        case CLOSE -> animation.setState(IDLE);
                        case RIGHT_SIDE_CLOSE -> animation.setState(RIGHT_SIDE_IDLE);
                        case LEFT_SIDE_CLOSE -> animation.setState(LEFT_SIDE_IDLE);
                    }
                }
            }
            case OPEN, RIGHT_SIDE_OPEN, LEFT_SIDE_OPEN -> {
                if (animation.getCurrentTextureIndex() != 0) {
                    animation.update();
                }
            }
            case IDLE, RIGHT_SIDE_IDLE, LEFT_SIDE_IDLE -> animation.update();
        }
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
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
        return physicsBody.getPhysicsTranslation();
    }

    public void setState(final boolean closed) {
        if (closed) {
            switch (direction) {
                case NORMAL -> animation.setState(CLOSE);
                case RIGHT_SIDE -> animation.setState(RIGHT_SIDE_CLOSE);
                case LEFT_SIDE -> animation.setState(LEFT_SIDE_CLOSE);
            }
            physicsBody.setType(null);
        } else {
            switch (direction) {
                case NORMAL -> animation.setState(OPEN);
                case RIGHT_SIDE -> animation.setState(RIGHT_SIDE_OPEN);
                case LEFT_SIDE -> animation.setState(LEFT_SIDE_OPEN);
            }
            animation.setCurrentFrame(3);
            physicsBody.setType(CollisionType.TRAP);
        }
    }

    public void destroy() {
        physicsBody.destroy();
    }
}
