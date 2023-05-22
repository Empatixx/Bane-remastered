package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;

public class Bat implements Enemy{
    private final static int DIMENSION_WIDTH = 64;
    private final static int DIMENSION_HEIGHT = 64;
    private final static float SCALE = 1.5f;
    private final static int DEFAULT_HEALTH = 9;
    private final Animation animation;
    private final Enemy enemyImpl;
    public Bat(){
        animation = Animation.builder()
                .setTotal(2,4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                .addState(AnimationState.DEATH, 1, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/bat.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        enemyImpl = new EnemyImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, SCALE, animation);
    }
    @Override
    public void update(double dt) {
        animation.update();
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public void onDeath() {

    }

    @Override
    public void onHit() {
    }

    @Override
    public void hit(final int damage) {
        enemyImpl.hit(damage);
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
    public boolean isDead() {
        return enemyImpl.isDead();
    }

    @Override
    public RenderData renderData() {
        return enemyImpl.renderData();
    }
    @Override
    public boolean canRender() {
        return enemyImpl.canRender();
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
