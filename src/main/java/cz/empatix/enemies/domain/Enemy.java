package cz.empatix.enemies.domain;

import cz.empatix.newphysics.attributes.DamageSource;
import cz.empatix.newphysics.attributes.Damageable;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;
public interface Enemy extends Drawable{
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
}
