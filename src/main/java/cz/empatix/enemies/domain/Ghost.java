package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.skills.Dash;
import cz.empatix.enemies.skills.Skill;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;

import java.util.List;

public class Ghost implements Enemy{
    private final static int MOVE_FORCE = 4_500_000;
    private final static int SCALE = 4;
    private final static int DIMENSION_WIDTH = (int) (22 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (28 * SCALE);
    private final static int DEFAULT_HEALTH = 11;
    private final static int DEFAULT_DAMAGE = 2;
    private final Animation animation;
    private final EnemyImpl enemyImpl;
    private final Skill dash;
    private int moveForce = MOVE_FORCE;

    public Ghost(){
        animation = Animation.builder()
                .setTotal(2,5)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.DEATH, 1, 5, DIMENSION_WIDTH, DIMENSION_HEIGHT, 85)
                .texture(AssetsService.getTexture("ingame/ghost.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        enemyImpl = new EnemyImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, animation,DEFAULT_DAMAGE, this::hit, () -> animation.hasPlayedOnce() && isDead(),
                () -> moveForce);
        dash = new Dash(() -> {}, () -> {}, force -> moveForce = moveForce + force);
    }
    @Override
    public void update(double dt) {
        animation.update();
        enemyImpl.update(dt);
        dash.update((float)dt);
        dash.activate();
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
    public boolean isDead() {
        return enemyImpl.isDead();
    }

    @Override
    public boolean canRemove() {
        return isDead() && animation.hasPlayedOnce() && Camera.isInProjection(getPosition(), getWidth(), getHeight());
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
        return !canRemove() && Camera.isInProjection(getPosition(), getWidth(), getHeight());
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
