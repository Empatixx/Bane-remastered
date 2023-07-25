package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

import java.util.List;

public class Golem implements Enemy {

    private final static int DIMENSION_WIDTH = 100;
    private final static int DIMENSION_HEIGHT = 100;
    private final static int SCALE = 5;
    private final static int DEFAULT_HEALTH = 130;
    private final Animation animation;
    private final Enemy bossImpl;
    public Golem(){
        animation = Animation.builder()
                .setTotal(8,14)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.SPELL0, 1, 8, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.SPELL1, 2, 9, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.SPELL2, 3, 8, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.ATTACK, 4, 7, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.SPELL3, 5, 7, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.SPELL4, 6, 10, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .addState(AnimationState.DEATH, 7, 14, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .texture(AssetsService.getTexture("ingame/golem.png"))
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
    public boolean isDead() {
        return bossImpl.isDead();
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    @Override
    public boolean canDropItems() {
        return bossImpl.canDropItems();
    }

    @Override
    public RenderData renderData() {
        return bossImpl.renderData();
    }
    @Override
    public List<Renderable> getSubRenderables() {
        return bossImpl.getSubRenderables();
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
    public Vector2f getPosition() {
        return bossImpl.getPosition();
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
