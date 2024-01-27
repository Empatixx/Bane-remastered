package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.skills.GreenSlimebullet;
import cz.empatix.enemies.skills.SlimeSmash;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;
import java.util.stream.Stream;

public class Slime implements Enemy {
    private final static int MOVE_FORCE = 500_000;
    private final static float SCALE = 3f;
    private final static int DIMENSION_WIDTH = (int) (32 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (24 * SCALE);
    private final static int COLLISION_OFFSET_X = (int) (0 * SCALE);
    private final static int COLLISION_OFFSET_Y = (int) (4 * SCALE);
    private final static Vector2i COLLISION_OFFSET = new Vector2i(COLLISION_OFFSET_X, COLLISION_OFFSET_Y);
    private final static int DEFAULT_HEALTH = 9;
    private final static int DEFAULT_DAMAGE = 1;
    private final Animation animation;
    private final EnemyImpl enemyImpl;
    private final SlimeSmash slimeSmash;

    public Slime() {
        animation = Animation.builder()
                .setTotal(2, 6)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.DEATH, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .texture(AssetsService.getTexture("ingame/slime.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        enemyImpl = new EnemyImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, animation, DEFAULT_DAMAGE, this::hit, () -> animation.hasPlayedOnce() && isDead(),
                () -> MOVE_FORCE, DIMENSION_WIDTH, DIMENSION_HEIGHT - COLLISION_OFFSET_Y, COLLISION_OFFSET);
        slimeSmash = new SlimeSmash(() -> new Vector2f(getX(), getY()).add(DIMENSION_WIDTH / 2, DIMENSION_HEIGHT / 2), GreenSlimebullet::new);
    }

    @Override
    public void update(double dt) {
        animation.update();
        slimeSmash.update((float) dt);
        enemyImpl.update(dt);
    }

    @Override
    public void setTarget(final Player player) {
        enemyImpl.setTarget(player);
    }


    @Override
    public void fixedUpdate(double dt) {
        enemyImpl.fixedUpdate(dt);
        if (isDead()) {
            return;
        }
        slimeSmash.activate();

    }

    private void onDeath() {
        animation.setState(AnimationState.DEATH);
        enemyImpl.onDeath();
    }

    @Override
    public void hit(final int damage) {
        if (isDead()) {
            return;
        }
        enemyImpl.hit(damage);
        if (isDead()) {
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
        return isDead() && animation.hasPlayedOnce() && slimeSmash.noBullets();
    }

    @Override
    public RenderData renderData() {
        return enemyImpl.renderData();
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return Stream.of(enemyImpl.getSubRenderables(), slimeSmash.getSubRenderables())
                .flatMap(List::stream)
                .toList();
    }

    @Override
    public boolean canRender() {
        return !isDead() || !animation.hasPlayedOnce();
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
