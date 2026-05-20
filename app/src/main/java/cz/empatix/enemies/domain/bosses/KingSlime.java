package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.enemies.domain.EnemyType;
import cz.empatix.enemies.domain.SpawnableEnemy;
import cz.empatix.enemies.skills.KingSlimeArmageddon;
import cz.empatix.enemies.skills.PurpleSlimebullet;
import cz.empatix.enemies.skills.SlimeSmash;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

@SpawnableEnemy(type = EnemyType.BOSS, minFloor = 2)
public class KingSlime implements Enemy {
    private final static int MOVE_FORCE = 10_000_000;
    private final static int SCALE = 4;
    private final static int DIMENSION_WIDTH = 64 * SCALE;
    private final static int DIMENSION_HEIGHT = 48 * SCALE;
    private final static int DEFAULT_HEALTH = 90;
    private final Animation animation;
    private final BossImpl bossImpl;
    private final KingSlimeArmageddon armageddon;
    private final SlimeSmash slimeSmash;

    public KingSlime() {
        animation = Animation.builder()
                .setTotal(2, 6)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.DEATH, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .texture(AssetsService.getTexture("ingame/kingslime.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        bossImpl = new BossImpl(DIMENSION_WIDTH, DIMENSION_HEIGHT, DEFAULT_HEALTH, animation, 2, this::hit, () -> MOVE_FORCE,getZIndex());
        armageddon = new KingSlimeArmageddon(() -> new Vector2f(getX(), getY()).add(DIMENSION_WIDTH / 2, DIMENSION_HEIGHT / 2), PurpleSlimebullet::new);
        slimeSmash = new SlimeSmash(() -> new Vector2f(getX(), getY()).add(DIMENSION_WIDTH / 2, DIMENSION_HEIGHT / 2), 25, PurpleSlimebullet::new);
    }

    @Override
    public void update(double dt) {
        animation.update();
        armageddon.update((float) dt);
        slimeSmash.update((float) dt);
        bossImpl.update(dt);
    }

    @Override
    public void fixedUpdate(double dt) {
        slimeSmash.fixedUpdate((float)dt);
        armageddon.fixedUpdate((float)dt);
        if (isDead()) {
            return;
        }
        bossImpl.fixedUpdate(dt);
        armageddon.activate();
        slimeSmash.activate();

    }


    @Override
    public int hit(final int damage) {
        if (isDead()) {
            return 0;
        }
        final int received = bossImpl.hit(damage);
        if (isDead()) {
            onDeath();
        }
        return received;
    }

    private void onDeath() {
        animation.setState(AnimationState.DEATH);
        bossImpl.onDeath();
    }


    @Override
    public void heal(final int health) {
        bossImpl.heal(health);
    }

    @Override
    public int getHealth() {
        return bossImpl.getHealth();
    }

    @Override
    public void onCollision() {

    }

    @Override
    public void onSpawn() {

    }

    @Override
    public boolean canDropItems() {
        return false;
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        bossImpl.collectSubRenderables(state);
        armageddon.collectSubRenderables(state);
        slimeSmash.collectSubRenderables(state);
    }

    @Override
    public boolean isDead() {
        return bossImpl.isDead();
    }

    @Override
    public boolean canRemove() {
        return isDead() && animation.hasPlayedOnce() && slimeSmash.noBullets() && armageddon.noBullets();
    }

    @Override
    public RenderData renderData() {
        return bossImpl.renderData();
    }

    @Override
    public boolean canRender() {
        return bossImpl.canRender() && !(isDead() && animation.hasPlayedOnce());
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

    @Override
    public void setTarget(final Player player) {
        bossImpl.setTarget(player);
    }

    @Override
    public void despawn() {
        armageddon.clear();
        slimeSmash.clear();
    }
}
