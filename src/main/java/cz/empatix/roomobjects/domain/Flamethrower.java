package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.attributes.DamageSource;
import cz.empatix.newphysics.attributes.DamageSourceType;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class Flamethrower implements RoomObject, DamageSource {
    private final static int SCALE = 4;
    private final static int DIMENSION_WIDTH = 16;
    private final static int DIMENSION_HEIGHT = 32;
    private final Animation animation;
    private final PhysicsObject physicsBody;
    private int damage;
    public Flamethrower(){
        animation = Animation.builder()
                .setTotal(1, 4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .texture(AssetsService.getTexture("ingame/flamethrower.tga"))
                .scale(SCALE)
                .build();
        damage = 1;
        physicsBody = PhysicsObjectFactory.create(DIMENSION_WIDTH * SCALE, DIMENSION_HEIGHT * SCALE);
        physicsBody.setType(CollisionType.TRAP);
        physicsBody.setUserData(this);
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
        return Camera.isInProjection(physicsBody.getPhysicsTranslation(), getWidth(), getHeight());
    }

    @Override
    public void update(double dt) {
        damage = animation.getCurrentTextureIndex() != 3 ? 0 : 1;
        animation.update();
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
    }
    public int getWidth() {
        return DIMENSION_WIDTH * SCALE;
    }

    public int  getHeight() {
        return DIMENSION_HEIGHT * SCALE;
    }

    @Override
    public void destroy() {
        physicsBody.destroy();
    }

    @Override
    public int damage() {
        return damage;
    }

    @Override
    public DamageSourceType type() {
        return DamageSourceType.TRAP;
    }

    public Vector2f getPosition() {
        return physicsBody.getPhysicsTranslation();
    }

    public PhysicsObject getPhysicsObject() {
        return physicsBody;
    }
}
