package cz.empatix.enemies.domain;

import cz.empatix.enemies.service.VitalityDisplayService;
import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.items.service.DroppableState;
import cz.empatix.newphysics.attributes.*;
import cz.empatix.newphysics.service.CategoryFilter;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.render.shaderbindings.OutlineBinding;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.joml.Vector4f;

import java.util.List;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class EnemyImpl implements Damageable, DamageSource {
    private final VitalityDisplayService vitalityDisplayService;
    private final Animation animation;
    private final PhysicsObject physicsBody;
    private final DroppableState droppableState;
    private final AStarTargetting aStarTargetting;
    private final int width;
    private final int height;
    private final int maxHealth;
    private final Supplier<Integer> moveForceSupplier;
    private int health;

    private long lastHit;
    private final int damage;

    public EnemyImpl(final int width,
                     final int height,
                     final int maxHealth,
                     final Animation animation,
                     final int damage,
                     final Consumer<Integer> hitCallback,
                     final Supplier<Boolean> droppableCondition,
                     final Supplier<Integer> moveForceSupplier) {
        this.maxHealth = health = maxHealth;
        this.width = width;
        this.height = height;
        this.animation = animation;
        this.moveForceSupplier = moveForceSupplier;
        this.damage = damage;

        physicsBody = PhysicsObjectFactory.create(width, height);
        physicsBody.getFixture(0).setFilter(CategoryFilter.ENEMY);
        physicsBody.setUserData(this);

        droppableState = new DroppableState(droppableCondition);
        aStarTargetting = new AStarTargetting(physicsBody.getPhysicsTranslation(), width, height);
        vitalityDisplayService = new VitalityDisplayService();
    }

    private boolean canHit(final DamageSourceType source, final int damage) {
        return source != DamageSourceType.ENEMY;
    }

    public void onDeath() {
        physicsBody.destroy();
    }

    public void hit(final int damage) {
        if (isDead()) {
            return;
        }
        vitalityDisplayService.addChange(-damage, getPosition(), getWidth(), getHeight());
        this.health -= damage;
        if (this.health < 0) {
            this.health = 0;
        }
        lastHit = System.currentTimeMillis();
    }

    public List<Renderable> getSubRenderables(){
        return vitalityDisplayService.getRenderables();
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

    public boolean canRemove() {
        return isDead() && vitalityDisplayService.isEmpty();
    }

    public boolean canDropItems() {
        return droppableState.canDrop();
    }

    public float getX() {
        return physicsBody.getPhysicsTranslationX();
    }

    public float getY() {
        return physicsBody.getPhysicsTranslationY();
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    public void setPosition(float x, float y) {
        physicsBody.setTranslation(x, y);
    }

    public Vector2f getPosition() {
        return physicsBody.getPhysicsTranslation();
    }

    public RenderData renderData() {
        final float outline = 1 - (System.currentTimeMillis() - lastHit) / 1000f;
        if (outline > 0) {
            return new RenderData(
                    animation.getVertices(physicsBody.getPhysicsTranslation(), aStarTargetting.isTargetLeftFromEnemy()),
                    animation.getTextureId(),
                    new OutlineBinding(Camera.getProjection(), new Vector2f(getWidth(), getHeight()),
                            animation.getNormalStartPositions(), animation.getNormalEndPositions(), new Vector4f(1, 0, 0, outline))
            );
        } else {
            return new RenderData(
                    animation.getVertices(physicsBody.getPhysicsTranslation(), aStarTargetting.isTargetLeftFromEnemy()),
                    animation.getTextureId(),
                    new CommonBinding(Camera.getProjection())
            );
        }
    }

    public boolean canRender() {
        final Vector2f cameraPos = Camera.getPosition();
        final Vector2f cameraDimensions = new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight());
        final Vector2f position = new Vector2f(getX(), getY());

        return position.x() + getWidth() > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + getHeight() > cameraPos.y()
                && position.y() < cameraPos.y() + cameraDimensions.y();
    }

    public void setTarget(final Player player) {
        aStarTargetting.target(player);
    }

    public void fixedUpdate(final double dt) {
        aStarTargetting.updatePath();
        aStarTargetting.update((float) dt);
        final float moveForce = moveForceSupplier.get();
        final Vector2f force = aStarTargetting.getDirectionTarget().mul(moveForce, moveForce);
        physicsBody.applyForce(force.x(), force.y());
    }

    public void update(final double dt) {
        aStarTargetting.update((float) dt);
        vitalityDisplayService.update(dt);
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
    public void hit(DamageSourceType source, int damage) {
        if (canHit(source, damage)) {
            hit(damage);
        }
    }
}
