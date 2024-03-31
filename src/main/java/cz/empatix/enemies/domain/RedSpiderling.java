package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

@SpawnableEnemy(minFloor = 2)
public class RedSpiderling extends BaseEnemy {
    private static final int SCALE = 3;
    private static final int HEIGHT = 20 * SCALE;
    private static final int WIDTH = 32 * SCALE;
    private static final int DEFAULT_DAMAGE = 2;
    private static final int DEFAULT_HEALTH = 3;
    private static final int MOVE_FORCE = 5_500_000;

    public RedSpiderling() {
        super(WIDTH, HEIGHT);
        animation = Animation.builder()
                .setTotal(2, 5)
                .addState(AnimationState.IDLE, 0, 5, WIDTH, HEIGHT, 120)
                .addState(AnimationState.DEATH, 1, 3, WIDTH, HEIGHT, 150)
                .texture(AssetsService.getTexture("ingame/spiderling.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
    }

    @Override
    public void onDeath() {
        animation.setState(AnimationState.DEATH);
    }

    public void removeSpawning() {
        spawnTime -= SPAWN_INVINCIBILITY;
    }

    @Override
    public void onSpawn() {

    }

    @Override
    public void postDamage(final int damage) {
        if (damage == 0) {
            return;
        }
        heal(1);
    }
}
