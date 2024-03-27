package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.BaseEnemy;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;

@SpawnableEnemy
public class Spider extends BaseEnemy {
    private static final int SCALE = 3;
    private static final int HEIGHT = 40 * SCALE;
    private static final int WIDTH = 64 * SCALE;
    private static final int DEFAULT_DAMAGE = 1;
    private static final int DEFAULT_HEALTH = 7;
    private static final int MOVE_FORCE = 32_500_000;
    public Spider() {
        super(WIDTH, HEIGHT);
        animation = Animation.builder()
                .setTotal(1,5)
                .addState(AnimationState.IDLE, 0, 5, WIDTH, HEIGHT, 120)
                .texture(AssetsService.getTexture("ingame/spider.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
    }
    @Override
    public void onDeath() {

    }

    @Override
    public void onSpawn() {

    }
}
