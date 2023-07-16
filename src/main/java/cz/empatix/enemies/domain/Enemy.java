package cz.empatix.enemies.domain;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderScene;

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
    int getWidth();
    int getHeight();
    void setPosition(float x, float y);
}
