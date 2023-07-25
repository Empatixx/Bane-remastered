package cz.empatix.enemies.domain;

import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

public interface Enemy extends Drawable {
    void update(double dt);
    void fixedUpdate(double dt);
    void hit(final int damage);
    void heal(final int health);
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
        return 2;
    }
    float getX();
    float getY();
    Vector2f getPosition();
    int getWidth();
    int getHeight();
    void setPosition(float x, float y);
    default void setTarget(Player player){

    }
}
