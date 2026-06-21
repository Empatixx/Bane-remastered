package cz.empatix.enemies.domain;

import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.collision.DamageSource;
import cz.empatix.collision.Damageable;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.shadow.domain.ShadowCaster;
import org.joml.Vector2f;
public interface Enemy extends Drawable, Targetable, ShadowCaster {
    void update(final double dt);
    void fixedUpdate(final double dt);
    int hit(final int damage);
    void heal(final int health);
    int getHealth();
    @Deprecated
    void onCollision();
    void onSpawn();
    boolean isDead();
    boolean canRemove();
    boolean canDropItems();
    @Override
    default RenderScene renderScene(){
        return RenderScene.SCENE;
    }
    @Override
    default int getZIndex(){
        return 3;
    }
    float getX();
    float getY();
    Vector2f getPosition();
    int getWidth();
    int getHeight();
    void setPosition(final float x,final float y);
    void setTarget(final Player player);
    default void despawn(){
        hit(getHealth());
    }

    /** Scales this enemy's stats for the given floor. Default is a no-op; common enemies override. */
    default void scaleToFloor(final int floor) {
    }

    // ShadowCaster default implementations
    @Override
    default Vector2f getShadowPosition() {
        return getPosition();
    }

    @Override
    default int getShadowWidth() {
        return getWidth();
    }

    @Override
    default int getShadowHeight() {
        return getHeight();
    }
}
