package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

public class RedSlime implements Enemy{
    private final static int MOVE_FORCE = 3000000;
    private final static int DIMENSION_WIDTH = 64;
    private final static int DIMENSION_HEIGHT = 48;
    private final static float SCALE = 1.5f;
    private final static int DEFAULT_HEALTH = 9;
    private final static int DEFAULT_DAMAGE = 1;
    private final Animation animation;
    private final EnemyImpl enemyImpl;
    public RedSlime(){
        animation = Animation.builder()
                .setTotal(2,6)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.DEATH, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .texture(AssetsService.getTexture("ingame/redslime.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        enemyImpl = new EnemyImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, SCALE, animation, DEFAULT_DAMAGE, this::hit, () -> animation.hasPlayedOnce() && isDead(),
                () -> MOVE_FORCE);
    }
    @Override
    public void update(double dt) {
        animation.update();
        enemyImpl.update(dt);
    }

    @Override
    public void setTarget(final Player player) {
        enemyImpl.setTarget(player);
    }

    @Override
    public void fixedUpdate(double dt) {
        enemyImpl.fixedUpdate(dt);
    }

    private void onDeath() {
        animation.setState(AnimationState.DEATH);
        enemyImpl.onDeath();
    }

    @Override
    public void hit(final int damage) {
        if(isDead()){
            return;
        }
        enemyImpl.hit(damage);
        if(isDead()){
            onDeath();
        }
    }

    @Override
    public void heal(final int health) {
        enemyImpl.heal(health);
    }

    @Override
    public void onCollision() {

    }

    @Override
    public void onSpawn() {

    }
    @Override
    public boolean canDropItems() {
        return enemyImpl.canDropItems();
    }


    @Override
    public boolean isDead() {
        return enemyImpl.isDead();
    }

    @Override
    public boolean canRemove() {
        return isDead() && animation.hasPlayedOnce();
    }

    @Override
    public RenderData renderData() {
        return enemyImpl.renderData();
    }
    @Override
    public boolean canRender() {
        return !canRemove();
    }
    @Override
    public float getX() {
        return enemyImpl.getX();
    }

    @Override
    public float getY() {
        return enemyImpl.getY();
    }
    @Override
    public Vector2f getPosition() {
        return enemyImpl.getPosition();
    }
    @Override
    public int getWidth() {
        return enemyImpl.getWidth();
    }

    @Override
    public int getHeight() {
        return enemyImpl.getHeight();
    }

    @Override
    public void setPosition(float x, float y) {
        enemyImpl.setPosition(x, y);
    }
}
