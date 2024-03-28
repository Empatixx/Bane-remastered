package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

@SpawnableEnemy
public class PurpleSpiderling extends BaseEnemy {
    private static final int SCALE = 3;
    private static final int HEIGHT = 20 * SCALE;
    private static final int WIDTH = 32 * SCALE;
    private static final int DEFAULT_DAMAGE = 1;
    private static final int DEFAULT_HEALTH = 3;
    private static final int MOVE_FORCE = 12_500_000;
    private static final ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    public PurpleSpiderling() {
        super(WIDTH, HEIGHT);
        animation = Animation.builder()
                .setTotal(1,5)
                .addState(AnimationState.IDLE, 0, 5, WIDTH, HEIGHT, 120)
                .texture(AssetsService.getTexture("ingame/purple_spiderling.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
    }
    @Override
    public void onDeath() {

    }

    public void removeSpawning() {
        spawnTime -= SPAWN_INVINCIBILITY;
    }

    @Override
    public void onSpawn() {

    }
}
