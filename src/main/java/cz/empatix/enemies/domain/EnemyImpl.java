package cz.empatix.enemies.domain;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.geometry.Rectangle;
import cz.empatix.physics.events.consumers.DamageConsumer;
import cz.empatix.physics.events.domain.DamageSource;
import cz.empatix.physics.events.suppliers.DamageSupplier;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

import java.util.function.Consumer;

public class EnemyImpl implements Enemy {
    private final Animation animation;
    private final PhysicsBody physicsBody;
    private final int width;
    private final int height;
    private final float scale;
    private final int maxHealth;
    private int health;

    public EnemyImpl(final int width,
                     final int height,
                     final int maxHealth,
                     final float scale,
                     final Animation animation,
                     final int damage,
                     final Consumer<Integer> hitCallback) {
        this.maxHealth = health = maxHealth;
        this.width = width;
        this.height = height;
        this.scale = scale;
        this.animation = animation;
        physicsBody = PhysicsBodyFactory.createDynamic(Rectangle.from((int) (width * scale), (int) (height * scale)));
        physicsBody.addEventSupplier(new DamageSupplier(DamageSource.ENEMY, () -> damage));
        physicsBody.addEventConsumer(new DamageConsumer(hitCallback, this::canHit));
    }

    private boolean canHit(final DamageSource source,final int damage) {
        return source != DamageSource.ENEMY;
    }

    @Override
    public void update(double dt) {
        throw new UnsupportedOperationException();
    }

    @Override
    public void fixedUpdate(double dt) {
        throw new UnsupportedOperationException();
    }

    @Override
    public void onDeath() {
        physicsBody.destroy();
    }

    @Override
    public void onHit() {
        throw new UnsupportedOperationException();
    }

    @Override
    public void hit(final int damage) {
        this.health -= damage;
        if (this.health < 0) {
            this.health = 0;
        }
    }

    @Override
    public void heal(final int health) {
        this.health += health;
        if (this.health > maxHealth) {
            this.health = maxHealth;
        }
    }

    @Override
    public void onCollision() {
        throw new UnsupportedOperationException();
    }

    @Override
    public void onSpawn() {
        throw new UnsupportedOperationException();
    }

    @Override
    public boolean isDead() {
        return health <= 0;
    }

    @Override
    public boolean canRemove() {
        return isDead();
    }

    @Override
    public float getX() {
        return physicsBody.getPhysicsTranslationX();
    }

    @Override
    public float getY() {
        return physicsBody.getPhysicsTranslationY();
    }

    @Override
    public int getWidth() {
        return (int) (width * scale);
    }

    @Override
    public int getHeight() {
        return (int) (height * scale);
    }

    @Override
    public void setPosition(float x, float y) {
        physicsBody.setTranslation(x, y);
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
    public boolean canRender() {
        final Vector2f cameraPos = Camera.getPosition();
        final Vector2f cameraDimensions = new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight());
        final Vector2f position = new Vector2f(getX(), getY());

        return position.x() + getWidth() > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + getHeight() > cameraPos.y()
                && position.y() < cameraPos.y() + cameraDimensions.y();
    }
}
