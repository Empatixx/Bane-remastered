package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

import java.util.List;

public class Rat implements Enemy{
    private final static int MOVE_FORCE = 7500000;
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int)(85 * SCALE);
    private final static int DIMENSION_HEIGHT = (int)(37* SCALE);
    private final static int DEFAULT_HEALTH = 12;
    private final static int DEFAULT_DAMAGE = 2;
    private final Animation animation;
    private final EnemyImpl enemyImpl;
    public Rat(){
        animation = Animation.builder()
                .setTotal(2,4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 120)
                .addState(AnimationState.DEATH, 1, 3, DIMENSION_WIDTH, DIMENSION_HEIGHT, 150)
                .texture(AssetsService.getTexture("ingame/rat.tga"))
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
    public void fixedUpdate(double dt) {
        enemyImpl.fixedUpdate(dt);
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return enemyImpl.getSubRenderables();
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
