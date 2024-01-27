package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

import java.util.List;

public class Bat implements Enemy{
    private final static int MOVE_FORCE = 7_500_000;
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int) (64 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (64 * SCALE);
    private final static int DEFAULT_HEALTH = 9;
    private final static int DEFAULT_DAMAGE = 2;
    private final Animation animation;
    private final EnemyImpl enemyImpl;
    public Bat(){
        animation = Animation.builder()
                .setTotal(2,4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                .addState(AnimationState.DEATH, 1, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/bat.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        enemyImpl = new EnemyImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, animation, DEFAULT_DAMAGE, this::hit, () -> animation.hasPlayedOnce() && isDead(),
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
    public List<Renderable> getSubRenderables() {
        return enemyImpl.getSubRenderables();
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
    public boolean isDead() {
        return enemyImpl.isDead();
    }

    @Override
    public boolean canRemove() {
        return isDead() && animation.hasPlayedOnce();
    }

    @Override
    public boolean canDropItems() {
        return enemyImpl.canDropItems();
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
