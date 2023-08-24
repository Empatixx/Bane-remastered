package cz.empatix.enemies.domain;

import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.items.service.DroppableState;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.geometry.Rectangle;
import cz.empatix.physics.events.consumers.DamageConsumer;
import cz.empatix.physics.events.domain.DamageSource;
import cz.empatix.physics.events.suppliers.DamageSupplier;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.EnemyBinding;
import org.joml.Vector2f;

import java.util.function.Consumer;
import java.util.function.Supplier;

public class EnemyImpl {
    private final Animation animation;
    private final PhysicsBody physicsBody;
    private final DroppableState droppableState;
    private final AStarTargetting aStarTargetting;
    private final int width;
    private final int height;
    private final float scale;
    private final int maxHealth;
    private final Supplier<Integer> moveForceSupplier;
    private int health;

    // shaders vars
    private final Vector2f texStartPos;
    private final Vector2f texEndPos;

    public EnemyImpl(final int width,
                     final int height,
                     final int maxHealth,
                     final float scale,
                     final Animation animation,
                     final int damage,
                     final Consumer<Integer> hitCallback,
                     final Supplier<Boolean> droppableCondition,
                     final Supplier<Integer> moveForceSupplier) {
        this.maxHealth = health = maxHealth;
        this.width = width;
        this.height = height;
        this.scale = scale;
        this.animation = animation;
        this.moveForceSupplier = moveForceSupplier;

        texStartPos = animation.getTexture().normalizedStartPositions();
        texEndPos = animation.getTexture().normalizedEndPositions();

        physicsBody = PhysicsBodyFactory.createDynamic(Rectangle.from((int) (width * scale), (int) (height * scale)));
        physicsBody.addEventSupplier(new DamageSupplier(DamageSource.ENEMY, () -> damage));
        physicsBody.addEventConsumer(new DamageConsumer(hitCallback, this::canHit));
        droppableState = new DroppableState(droppableCondition);
        aStarTargetting = new AStarTargetting(physicsBody.getTranslation(), (int) (width * scale), (int) (height * scale));
    }

    private boolean canHit(final DamageSource source,final int damage) {
        return source != DamageSource.ENEMY;
    }

    public void onDeath() {
        physicsBody.destroy();
    }

    public void hit(final int damage) {
        this.health -= damage;
        if (this.health < 0) {
            this.health = 0;
        }
    }

    public void heal(final int health) {
        this.health += health;
        if (this.health > maxHealth) {
            this.health = maxHealth;
        }
    }

    public boolean isDead() {
        return health <= 0;
    }

    public boolean canRemove() {
        return isDead();
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
        return (int) (width * scale);
    }

    public int getHeight() {
        return (int) (height * scale);
    }

    public void setPosition(float x, float y) {
        physicsBody.setTranslation(x, y);
    }

    public Vector2f getPosition() {
        return physicsBody.getTranslation();
    }

    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getTranslation(), aStarTargetting.isTargetLeftFromEnemy()),
                animation.getTextureId(),
                new EnemyBinding(Camera.getProjection(), new Vector2f(getWidth(), getHeight()), texStartPos, texEndPos)
        );
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
        aStarTargetting.update((float)dt);
        final float moveForce = moveForceSupplier.get();
        final Vector2f force = aStarTargetting.getDirectionTarget().mul(moveForce,moveForce);
        physicsBody.applyForce(force.x(), force.y());
    }

    public void update(final double dt) {
        aStarTargetting.update((float)dt);
    }
}
