package cz.empatix.pets;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.interaction.domain.InteractEntity;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.newphysics.attributes.*;
import cz.empatix.collision.DamageSource;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.collision.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.particles.service.GazewindService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.animation.AnimationState;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.shadow.domain.ShadowCaster;
import org.joml.Vector2f;

import java.util.HashSet;
import java.util.Set;

public class Pet implements InteractEntity, Drawable, Targetable, DamageSource, ShadowCaster {
    private final static Set<Integer> INTERACT_KEYS = new HashSet<>();
    private final static int GAZE_OFFSET = 16;
    private final static int GAZE_SPEED_PRELIMIT = 150;
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
    private final GazewindService gazewindService;
    public Pet() {
        animation = Animation.builder()
                .texture(AssetsService.getTexture("ingame/wolf_pet.tga"))
                .setTotal(4, 4)
                .addState(AnimationState.BOTTOM_MOVE, 0, 4, WIDTH, HEIGHT, 100)
                .addState(AnimationState.TOP_MOVE, 1, 4, WIDTH, HEIGHT, 100)
                .addState(AnimationState.RIGHT_MOVE, 2, 4, WIDTH, HEIGHT, 100)
                .addState(AnimationState.LEFT_MOVE, 2, 4, WIDTH, HEIGHT, 100, true)
                .addState(AnimationState.IDLE, 3, 2, WIDTH, HEIGHT, 250)
                .build();
        animation.setState(AnimationState.IDLE);
        physicsObject = PhysicsObjectFactory.createInterpolated(WIDTH, HEIGHT);
        physicsObject.setType(CollisionType.PLAYER);
        physicsObject.setUserData(this);

        followingStrategy = new OwnerFollowingStrategy(this);
        collectingStrategy = new DropFollowingStrategy(this);
        attackingStrategy = new AttackFollowingStrategy(this);
        InteractionService.getInstance().addEntity(this);

        gazewindService = new GazewindService();

    }
    public void setOwner(final Targetable owner) {
        followingStrategy.follow(owner);
    }

    public void update(final float dt) {
        animation.update();
        followingStrategy.update(dt);
        attackingStrategy.update(dt);
        collectingStrategy.update(dt);
        gazewindService.update((float) dt);
    }

    public void fixedUpdate(final float dt) {
        followingStrategy.fixedUpdate(dt);
        attackingStrategy.fixedUpdate(dt);
        collectingStrategy.fixedUpdate(dt);
        move();
        trySpawnGazeWind();
    }

    private void trySpawnGazeWind() {
        final Vector2f velocity = physicsObject.getVelocity();
        if (Math.abs(velocity.x()) >= GAZE_SPEED_PRELIMIT || Math.abs(velocity.y()) >= GAZE_SPEED_PRELIMIT) {
            final Vector2f nextGazePosition = getGazeWindPosition();
            gazewindService.spawn(nextGazePosition.x(), nextGazePosition.y());
        }

    }

    public Vector2f getGazeWindPosition() {
        final Vector2f position = new Vector2f(physicsObject.getTranslation());
        position.x += WIDTH / 2;
        position.y += HEIGHT;
        return position;
    }

    private void move(){
        final Vector2f direction;
        if (collectingStrategy.hasTarget()) {
            direction = collectingStrategy.getDirection();
        } else if (attackingStrategy.hasTarget()) {
            direction = attackingStrategy.getDirection();
        } else {
            direction = followingStrategy.getDirection();
        }
        setAnimation(direction);
        direction.mul(MOVE_FORCE);
        physicsObject.applyForce(direction.x(), direction.y());
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
        return INTERACT_KEYS;
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
                animation.getVertices(physicsObject.getTranslation(), getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
        );
    }
    @Override
    public void collectSubRenderables(RenderableState state) {
        gazewindService.collectSubRenderables(state);
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
        return Camera.getInstance().isInProjection(physicsObject.getTranslation(), WIDTH, HEIGHT);
    }

    public void setPosition(float x, float y) {
        physicsObject.setPhysicsTranslation(x, y);
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

    // ShadowCaster implementations
    @Override
    public Vector2f getShadowPosition() {
        return physicsObject.getTranslation();
    }

    @Override
    public int getShadowWidth() {
        return getWidth();
    }

    @Override
    public int getShadowHeight() {
        return getHeight();
    }

    @Override
    public float getShadowFeetOffsetY() {
        return -HEIGHT * 0.3f;
    }
}
