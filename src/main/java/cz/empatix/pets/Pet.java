package cz.empatix.pets;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.interaction.domain.InteractEntity;
import cz.empatix.newphysics.attributes.*;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.EnumSet;
import java.util.List;
import java.util.Set;

public class Pet implements InteractEntity, Drawable, Targetable, DamageSource {
    private final static int DAMAGE_COOLDOWN = 200;
    private final static int DAMAGE = 1;
    private static final int MOVE_FORCE = 12_500_000;
    private static final int SCALE = 3;
    private static final int WIDTH = 32 * SCALE;
    private static final int HEIGHT = 32 * SCALE;
    private static final int AREA_ATTACK = 500;
    private final InterpolatedPhysicsObject physicsObject;
    private final Animation animation;
    private final FollowingStrategy followingStrategy;
    private final FollowingStrategy collectingStrategy;
    private final FollowingStrategy attackingStrategy;
    private long lastAttack = 0;

    public Pet() {
        animation = Animation.builder()
                .texture(AssetsService.getTexture("ingame/wolf_pet.tga"))
                .setTotal(3, 4)
                .addState(AnimationState.BOTTOM_MOVE, 0, 4, WIDTH, HEIGHT, 100)
                .addState(AnimationState.TOP_MOVE, 1, 4, WIDTH, HEIGHT, 100)
                .addState(AnimationState.RIGHT_MOVE, 2, 4, WIDTH, HEIGHT, 100)
                .addState(AnimationState.LEFT_MOVE, 2, 4, WIDTH, HEIGHT, 100, true)
                .addState(AnimationState.IDLE, 0, 1, WIDTH, HEIGHT, 100)
                .build();
        animation.setState(AnimationState.IDLE);
        physicsObject = PhysicsObjectFactory.createInterpolated(WIDTH, HEIGHT);
        physicsObject.setType(CollisionType.PLAYER);
        physicsObject.setUserData(this);
        followingStrategy = new OwnerFollowingStrategy(this);
        collectingStrategy = new FollowingStrategyImpl(this);
        attackingStrategy = new AttackFollowingStrategy(this);
    }
    public void setOwner(final Targetable owner) {
        followingStrategy.follow(owner);
    }

    public void update(final float dt) {
        physicsObject.update(dt);
        animation.update();
        followingStrategy.update(dt);
        attackingStrategy.update(dt);
    }

    public void fixedUpdate(final float dt) {
        followingStrategy.fixedUpdate(dt);
        attackingStrategy.fixedUpdate(dt);

        if (attackingStrategy.hasTarget()){
            final Vector2f moveDirection = attackingStrategy.getDirection();
            setAnimation(moveDirection);
            moveDirection.mul(MOVE_FORCE);
            physicsObject.applyForce(moveDirection.x(), moveDirection.y());
            return;
        } else if (collectingStrategy.hasTarget()){
            final Vector2f moveDirection = collectingStrategy.getDirection();
            setAnimation(moveDirection);
            moveDirection.mul(MOVE_FORCE);
            physicsObject.applyForce(moveDirection.x(), moveDirection.y());
            return;
        } else {
            final Vector2f moveDirection = followingStrategy.getDirection();
            setAnimation(moveDirection);
            moveDirection.mul(MOVE_FORCE);
            physicsObject.applyForce(moveDirection.x(), moveDirection.y());
        }
    }
    private void setAnimation(Vector2f direction) {
        if (direction.x() > 0.3f) {
            animation.setState(AnimationState.RIGHT_MOVE);
        } else if (direction.x() < -0.3f) {
            animation.setState(AnimationState.LEFT_MOVE);
        } else if (direction.y() > 0f) {
            animation.setState(AnimationState.BOTTOM_MOVE);
        } else if (direction.y() < 0f) {
            animation.setState(AnimationState.TOP_MOVE);
        } else {
            animation.setState(AnimationState.IDLE);
        }
    }

    @Override
    public Set<Integer> getInteractKeys() {
        return Set.of();
    }

    @Override
    public Vector2f getPosition() {
        return physicsObject.getPhysicsTranslation();
    }

    @Override
    public int getHeight() {
        return HEIGHT;
    }

    @Override
    public int getWidth() {
        return WIDTH;
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsObject.getTranslation()),
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
        return 3;
    }

    @Override
    public boolean canRender() {
        return Camera.isInProjection(physicsObject.getTranslation(), WIDTH, HEIGHT);
    }

    public void setPosition(float x, float y) {
        physicsObject.setTranslation(x, y);
    }

    @Override
    public int damage() {
        if (System.currentTimeMillis() - lastAttack < DAMAGE_COOLDOWN) {
            return 0;
        }
        return DAMAGE;
    }

    @Override
    public void postDamage(final int damage) {
        lastAttack = System.currentTimeMillis();
    }

    public PhysicsObject getPhysicsObject() {
        return physicsObject;
    }

    @Override
    public DamageSourceType type() {
        return DamageSourceType.PLAYER;
    }
}
