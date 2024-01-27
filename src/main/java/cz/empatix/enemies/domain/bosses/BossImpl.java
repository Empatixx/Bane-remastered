package cz.empatix.enemies.domain.bosses;

import cz.empatix.common.handlers.WindowHandler;
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
import org.joml.Vector2f;

import java.util.List;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class BossImpl implements Damageable, DamageSource {
    private final AStarTargetting aStarTargetting;
    private final StatusBar healthBar;
    private final Animation animation;
    private final PhysicsObject physicsBody;
    private final int width;
    private final int height;
    private final int maxHealth;
    private int health;
    private int damage;
    private final Supplier<Integer> moveForceSupplier;
    private final Consumer<Integer> hitCallback;

    public BossImpl(final int width,
                    final int height,
                    final int maxHealth,
                    final Animation animation,
                    final int damage,
                    final Consumer<Integer> hitCallback,
                    final Supplier<Integer> moveForceSupplier) {
        this.maxHealth = health = maxHealth;
        this.width = width;
        this.height = height;
        this.animation = animation;
        this.moveForceSupplier = moveForceSupplier;
        this.hitCallback = hitCallback;
        this.damage = damage;


        physicsBody = PhysicsObjectFactory.createInterpolated(width, height);
        physicsBody.setType(CollisionType.ENEMY);
        physicsBody.setUserData(this);

        aStarTargetting = new AStarTargetting(physicsBody::getPhysicsTranslation, width, height);

        final float xPosition = WindowHandler.TARGET_WIDTH / 2f - StatusBarType.BOSS_HEALTH.getTexture().width() * StatusBarType.BOSS_HEALTH.getScale() / 2f;
        healthBar = new StatusBar(() -> health, () -> maxHealth, StatusBarType.BOSS_HEALTH, new Vector2f(xPosition, 450));
    }

    private boolean canHit(final DamageSourceType source, final int damage) {
        return source != DamageSourceType.ENEMY;
    }

    public void hit(final int damage) {
        this.health -= damage;
        if (this.health < 0) {
            this.health = 0;
        }
    }

    public void update(final double dt) {
        aStarTargetting.update((float) dt);
        healthBar.update(dt);
    }

    public void onDeath() {
        physicsBody.destroy();
    }

    public void fixedUpdate(final double dt){
        aStarTargetting.updatePath();
        aStarTargetting.update((float) dt);
        final float moveForce = moveForceSupplier.get();
        final Vector2f force = aStarTargetting.getDirectionTarget().mul(moveForce, moveForce);
        physicsBody.applyForce(force.x(), force.y());
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

    public void setPosition(float x, float y) {
        physicsBody.setPhysicsTranslation(x, y);
    }

    public List<Renderable> getSubRenderables() {
        if (isDead()){
            return List.of();
        }
        return List.of(healthBar);
    }

    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getPhysicsTranslation(), aStarTargetting.isTargetLeftFromEnemy()),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
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
            hitCallback.accept(damage);
        }
    }
}
