package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;

import java.util.List;

public class KingSlime implements Enemy {
    private final static int DIMENSION_WIDTH = 64;
    private final static int DIMENSION_HEIGHT = 48;
    private final static int SCALE = 5;
    private final static int DEFAULT_HEALTH = 90;
    private final Animation animation;
    private final Enemy bossImpl;
    public KingSlime(){
        animation = Animation.builder()
                .setTotal(2,6)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.DEATH, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .texture(AssetsService.getTexture("ingame/kingslime.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        bossImpl = new BossImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, SCALE, animation);
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
        bossImpl.hit(damage);
    }

    @Override
    public void heal(final int health) {
        bossImpl.heal(health);
    }

    @Override
    public void onCollision() {

    }

    @Override
    public void onSpawn() {

    }

    @Override
    public List<Drawable> getSubDrawables() {
        return bossImpl.getSubDrawables();
    }

    @Override
    public boolean isDead() {
        return bossImpl.isDead();
    }

    @Override
    public RenderData renderData() {
        return bossImpl.renderData();
    }
    @Override
    public boolean canRender() {
        return bossImpl.canRender();
    }
    @Override
    public float getX() {
        return bossImpl.getX();
    }

    @Override
    public float getY() {
        return bossImpl.getY();
    }

    @Override
    public int getWidth() {
        return bossImpl.getWidth();
    }

    @Override
    public int getHeight() {
        return bossImpl.getHeight();
    }

    @Override
    public void setPosition(float x, float y) {
        bossImpl.setPosition(x, y);
    }
}
