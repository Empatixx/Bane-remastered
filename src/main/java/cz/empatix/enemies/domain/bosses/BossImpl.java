package cz.empatix.enemies.domain.bosses;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.enemies.service.VitalityDisplayService;
import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.hud.domain.StatusBarType;
import cz.empatix.hud.service.StatusBar;
import cz.empatix.newphysics.attributes.*;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.render.shaderbindings.OutlineBinding;
import org.dyn4j.geometry.MassType;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector4f;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.Supplier;

public class BossImpl implements Damageable, DamageSource {
    private final VitalityDisplayService vitalityDisplayService;
    private final AStarTargetting aStarTargetting;
    private final StatusBar healthBar;
    private final Animation animation;
    private final InterpolatedPhysicsObject physicsBody;
    private final int width;
    private final int height;
    private final int maxHealth;
    private int health;
    private int damage;
    private final Supplier<Integer> moveForceSupplier;
    private final Function<Integer, Integer> hitCallback;
    private long lastHit;
    private final int zDepth;
    public BossImpl(final int width,
                    final int height,
                    final int maxHealth,
                    final Animation animation,
                    final int damage,
                    final Function<Integer, Integer> hitCallback,
                    final Supplier<Integer> moveForceSupplier,
                    final int zDepth) {
        this.maxHealth = health = maxHealth;
        this.width = width;
        this.height = height;
        this.animation = animation;
        this.moveForceSupplier = moveForceSupplier;
        this.hitCallback = hitCallback;
        this.damage = damage;
        this.zDepth = zDepth;


        physicsBody = PhysicsObjectFactory.createInterpolated(width, height);
        physicsBody.setType(CollisionType.ENEMY);
        physicsBody.setUserData(this);

        aStarTargetting = new AStarTargetting(physicsBody::getPhysicsTranslation, width, height);

        final float xPosition = WindowHandler.TARGET_WIDTH / 2f - StatusBarType.BOSS_HEALTH.getTexture().width() * StatusBarType.BOSS_HEALTH.getScale() / 2f;
        healthBar = new StatusBar(() -> health, () -> maxHealth, StatusBarType.BOSS_HEALTH, new Vector2f(xPosition, 450));
        vitalityDisplayService = new VitalityDisplayService();
    }

    public BossImpl(final int width,
                    final int height,
                    final int maxHealth,
                    final Animation animation,
                    final int damage,
                    final Function<Integer, Integer> hitCallback,
                    final Supplier<Integer> moveForceSupplier,
                    final int collisionWidth,
                    final int collisionHeight,
                    final Vector2i collisionOffset,
                    final int zDepth) {
        this.maxHealth = health = maxHealth;
        this.width = width;
        this.height = height;
        this.animation = animation;
        this.moveForceSupplier = moveForceSupplier;
        this.hitCallback = hitCallback;
        this.damage = damage;
        this.zDepth = zDepth;


        physicsBody = PhysicsObjectFactory.createInterpolated(collisionWidth, collisionHeight);
        physicsBody.setMass(MassType.FIXED_ANGULAR_VELOCITY);
        physicsBody.setOffset(collisionOffset.x(), collisionOffset.y());
        physicsBody.setType(CollisionType.ENEMY);
        physicsBody.setUserData(this);

        aStarTargetting = new AStarTargetting(physicsBody::getPhysicsTranslation, width, height);

        final float xPosition = WindowHandler.TARGET_WIDTH / 2f - StatusBarType.BOSS_HEALTH.getTexture().width() * StatusBarType.BOSS_HEALTH.getScale() / 2f;
        healthBar = new StatusBar(() -> health, () -> maxHealth, StatusBarType.BOSS_HEALTH, new Vector2f(xPosition, 450));

        vitalityDisplayService = new VitalityDisplayService();
    }

    private boolean canHit(final DamageSourceType source, final int damage) {
        return source != DamageSourceType.ENEMY;
    }

    public int hit(final int damage) {
        if (isDead()) {
            return 0;
        }
        vitalityDisplayService.addChange(-damage, getPosition(), getWidth(), getHeight());
        this.health -= damage;
        if (this.health < 0) {
            this.health = 0;
        }
        lastHit = System.currentTimeMillis();
        return damage;
    }

    public void update(final double dt) {
        aStarTargetting.update((float) dt);
        healthBar.update(dt);
        vitalityDisplayService.update(dt);
    }

    public void onDeath() {
        physicsBody.destroy();
    }

    public void fixedUpdate(final double dt) {
        aStarTargetting.updatePath();
        aStarTargetting.update((float) dt);
        final float moveForce = moveForceSupplier.get();
        final Vector2f force = aStarTargetting.getDirectionTarget().mul(moveForce, moveForce);
        physicsBody.applyForce(force.x(), force.y());
    }

    public void heal(final int health) {
        if (isDead()) {
            return;
        }
        vitalityDisplayService.addChange(health, getPosition(), getWidth(), getHeight());
        this.health += health;
        if (this.health > maxHealth) {
            this.health = maxHealth;
        }
    }

    public boolean isDead() {
        return health <= 0;
    }


    public float getX() {
        return physicsBody.getPhysicsTranslationX();
    }

    public float getY() {
        return physicsBody.getPhysicsTranslationY();
    }

    public Vector2f getPosition() {
        return physicsBody.getPhysicsTranslation();
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    public void setPosition(final float x, final float y) {
        physicsBody.setPhysicsTranslation(x, y);
    }

    public List<Renderable> getSubRenderables() {
        if (isDead()) {
            return List.of();
        }
        final List<Renderable> renderables = new ArrayList<>();
        renderables.add(healthBar);
        renderables.addAll(vitalityDisplayService.getRenderables());
        return renderables;
    }

    public RenderData renderData() {
        final float outline = 1 - (System.currentTimeMillis() - lastHit) / 1000f;
        if (outline > 0) {
            return new RenderData(
                    animation.getVertices(physicsBody.getTranslation(), zDepth, aStarTargetting.isTargetLeftFromEnemy()),
                    animation.getTextureId(),
                    new OutlineBinding(new Vector2f(getWidth(), getHeight()),
                            animation.getNormalStartPositions(), animation.getNormalEndPositions(), new Vector4f(1, 0, 0, outline))
            );
        } else {
            return new RenderData(
                    animation.getVertices(physicsBody.getTranslation(), zDepth, aStarTargetting.isTargetLeftFromEnemy()),
                    animation.getTextureId(),
                    new CommonBinding()
            );
        }
    }

    public boolean isRightFacing() {
        return aStarTargetting.isTargetLeftFromEnemy();
    }

    public boolean canRender() {
        return Camera.getInstance().isInProjection(getPosition(), getWidth(), getHeight());
    }

    public void setTarget(final Player player) {
        aStarTargetting.target(player);
    }

    @Override
    public int damage() {
        return damage;
    }

    @Override
    public DamageSourceType type() {
        return DamageSourceType.ENEMY;
    }

    @Override
    public int hit(DamageSourceType source, int damage) {
        if (canHit(source, damage)) {
            return hitCallback.apply(damage);
        }
        return 0;
    }

    public PhysicsObject getPhysicsObject() {
        return physicsBody;
    }

    public int getHealth() {
        return health;
    }
}
