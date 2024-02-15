package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.Enemy;
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
    private final static Vector2i COLLISION_OFFSET = new Vector2i(COLLISION_OFFSET_X, COLLISION_OFFSET_Y);
    private final Animation animation;
    private final BossImpl bossImpl;
    private final Healing healing;

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
        bossImpl = new BossImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, animation, 4, this::hit, () -> MOVE_FORCE, COLLISION_WIDTH, COLLISION_HEIGHT, COLLISION_OFFSET);
        healing = new Healing(2, this::heal);
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
    public void hit(final int damage) {
        if (isDead()) {
            return;
        }
        bossImpl.hit(damage);
        if (isDead()) {
            onDeath();
        }
    }

    private void onDeath() {
        animation.setState(AnimationState.DEATH);
        bossImpl.onDeath();
    }

    @Override
    public void fixedUpdate(double dt) {
        if(isDead()){
            return;
        }
        bossImpl.fixedUpdate(dt);
        healing.activate();
        healing.update((float) dt);
        if (healing.isActive()) {
            if (animation.getCurrentState() != AnimationState.SPELL2) {
                animation.setState(AnimationState.SPELL2);
                final PhysicsObject physicsObject = bossImpl.getPhysicsObject();
                physicsObject.setMass(MassType.INFINITE);
                physicsObject.setVelocity(0, 0);
            }
        } else {
            animation.setState(AnimationState.IDLE);
            final PhysicsObject physicsObject = bossImpl.getPhysicsObject();
            if (physicsObject.getMass().getType() == MassType.INFINITE) {
                physicsObject.setMass(MassType.FIXED_ANGULAR_VELOCITY);
            }
        }

    }


    @Override
    public void heal(final int health) {
        bossImpl.heal(health);
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
        return bossImpl.getSubRenderables();
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
    }

    @Override
    public void setTarget(final Player player) {
        bossImpl.setTarget(player);
    }
}
