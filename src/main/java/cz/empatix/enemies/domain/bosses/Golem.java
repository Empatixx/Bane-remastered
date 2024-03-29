package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.enemies.domain.EnemyType;
import cz.empatix.enemies.domain.SpawnableEnemy;
import cz.empatix.enemies.skills.ArmorStacking;
import cz.empatix.enemies.skills.Dash;
import cz.empatix.enemies.skills.Healing;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import org.dyn4j.geometry.MassType;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.ArrayList;
import java.util.List;

public class Golem implements Enemy {
    private final static int MOVE_FORCE = 12_500_000;
    private final static int SCALE = 3;
    private final static int DIMENSION_WIDTH = 100 * SCALE;
    private final static int DIMENSION_HEIGHT = 100 * SCALE;
    private final static int DEFAULT_HEALTH = 100;
    private final static int COLLISION_WIDTH = 50 * SCALE;
    private final static int COLLISION_HEIGHT = 50 * SCALE;
    private final static int COLLISION_OFFSET_X = 25 * SCALE;
    private final static int COLLISION_OFFSET_Y = 25 * SCALE;
    private final static int EYE_OFFSET_X = 42 * SCALE;
    private final static int EYE_OFFSET_Y = 20 * SCALE;
    private final static Vector2i COLLISION_OFFSET = new Vector2i(COLLISION_OFFSET_X, COLLISION_OFFSET_Y);
    private final Animation animation;
    private final BossImpl bossImpl;
    private final Healing healing;
    private final Dash dash;
    private final ArmorStacking armorStacking;
    private final GolemLaser golemLaser;
    private int moveForce = MOVE_FORCE;

    public Golem() {
        animation = Animation.builder()
                .setTotal(8, 14)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.SPELL0, 1, 8, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.SPELL1, 2, 9, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.SPELL2, 3, 8, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.ATTACK, 4, 7, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.SPELL3, 5, 7, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.SPELL4, 6, 10, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.DEATH, 7, 14, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .texture(AssetsService.getTexture("ingame/golem.png"))
                .build();
        animation.setState(AnimationState.IDLE);
        bossImpl = new BossImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, animation, 4, this::hit, () -> moveForce, COLLISION_WIDTH, COLLISION_HEIGHT, COLLISION_OFFSET);
        golemLaser = new GolemLaser();
        healing = new Healing(2, this::heal);
        armorStacking = new ArmorStacking();
        dash = new Dash(() -> {}, () -> {}, force -> moveForce = moveForce + force);
    }


    @Override
    public void update(double dt) {
        bossImpl.update(dt);
        if (animation.getCurrentState() == AnimationState.SPELL2) {
            if (!animation.isLastFrame()) {
                animation.update();
            }
        } else {
            animation.update();
        }
    }

    @Override
    public int hit(final int damage) {
        if (isDead()) {
            return 0;
        }
        final int received = bossImpl.hit(damage);
        if (isDead()) {
            onDeath();
        }
        return received;
    }

    private void onDeath() {
        animation.setState(AnimationState.DEATH);
        bossImpl.onDeath();
        golemLaser.destroy();
    }

    @Override
    public void fixedUpdate(double dt) {
        golemLaser.fixedUpdate((float) dt);

        if (true){
            return;
        }
        if (isDead()) {
            return;
        }
        bossImpl.fixedUpdate(dt);
        if (bossImpl.isDead()) {
            return;
        }
        updateAnimationState(dt);

        activateArmorStacking();
        activateHealing();
        dash.activate();

        armorStacking.update((float) dt);
        healing.update((float) dt);
        dash.update((float) dt);
    }

    private void updateAnimationState(final double dt) {
        final AnimationState currentState = animation.getCurrentState();
        final AnimationState nextState = switch (currentState) {
            case SPELL2 -> {
                final AnimationState next = healing.isActive() ? AnimationState.SPELL2 : AnimationState.IDLE;
                if (next == AnimationState.IDLE) {
                    final PhysicsObject physicsObject = bossImpl.getPhysicsObject();
                    physicsObject.setMass(MassType.FIXED_ANGULAR_VELOCITY);
                }
                yield next;
            }
            case SPELL4 -> {
                yield animation.hasPlayedOnce() ? AnimationState.IDLE : AnimationState.SPELL4;
            }
            default -> currentState;
        };
        if (currentState != nextState) {
            animation.setState(nextState);

        }
    }


    private void activateHealing() {
        if (!animation.getCurrentState().equals(AnimationState.IDLE)) {
            return;
        }
        healing.activate();
        if (healing.isActive()) {
            animation.setState(AnimationState.SPELL2);
            final PhysicsObject physicsObject = bossImpl.getPhysicsObject();
            physicsObject.setMass(MassType.INFINITE);
            physicsObject.setVelocity(0, 0);
        }
    }

    private void activateArmorStacking() {
        if (!animation.getCurrentState().equals(AnimationState.IDLE)) {
            return;
        }
        armorStacking.activate();
        if (armorStacking.wasActivated()) {
            animation.setState(AnimationState.SPELL4);
        }
    }


    @Override
    public void heal(final int health) {
        bossImpl.heal(health);
    }

    @Override
    public int getHealth() {
        return bossImpl.getHealth();
    }

    @Override
    public void onCollision() {

    }

    @Override
    public void onSpawn() {

    }

    @Override
    public boolean isDead() {
        return bossImpl.isDead();
    }

    @Override
    public boolean canRemove() {
        return isDead() && animation.hasPlayedOnce();
    }

    @Override
    public boolean canDropItems() {
        return false;
    }

    @Override
    public RenderData renderData() {
        return bossImpl.renderData();
    }

    @Override
    public List<Renderable> getSubRenderables() {
        final List<Renderable> renderables = new ArrayList<>();
        final List<Renderable> bossImplRenderables = bossImpl.getSubRenderables();
        renderables.addAll(bossImplRenderables);
        renderables.add(golemLaser);
        return renderables;
    }

    @Override
    public boolean canRender() {
        return bossImpl.canRender();
    }

    @Override
    public float getX() {
        return bossImpl.getX();
    }

    @Override
    public float getY() {
        return bossImpl.getY();
    }

    @Override
    public Vector2f getPosition() {
        return bossImpl.getPosition();
    }

    @Override
    public int getWidth() {
        return bossImpl.getWidth();
    }

    @Override
    public int getHeight() {
        return bossImpl.getHeight();
    }

    @Override
    public void setPosition(float x, float y) {
        bossImpl.setPosition(x, y);
        golemLaser.setPosition(bossImpl.getX() + EYE_OFFSET_X, bossImpl.getY() + EYE_OFFSET_Y);

    }

    @Override
    public void setTarget(final Player player) {
        bossImpl.setTarget(player);
        golemLaser.setTarget(player);
    }
}
