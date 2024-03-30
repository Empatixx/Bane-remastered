package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.skills.SpiderlingTeleport;
import cz.empatix.player.Player;
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
    private static final int MOVE_FORCE = 6_500_000;
    private static final ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final SpiderlingTeleport teleport;
    public PurpleSpiderling() {
        super(WIDTH, HEIGHT);
        animation = Animation.builder()
                .setTotal(3,5)
                .addState(AnimationState.IDLE, 0, 5, WIDTH, HEIGHT, 120)
                .addState(AnimationState.DEATH, 1, 3, WIDTH, HEIGHT, 150)
                .addState(AnimationState.SPELL0, 2, 3, WIDTH, HEIGHT, 150)
                .addState(AnimationState.SPELL1, 2, 3, WIDTH, HEIGHT, 150, true) // reverse of spell0
                .texture(AssetsService.getTexture("ingame/purple_spiderling.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
        teleport = new SpiderlingTeleport(animation, this);
    }
    @Override
    public void update(final double dt){
        super.update(dt);
        if (isSpawning() || isDead()){
            return;
        }
        final AnimationState currentState = animation.getCurrentState();
        switch (currentState){
            case SPELL1 -> {
                if (animation.hasPlayedOnce()){
                    animation.setState(AnimationState.IDLE);
                }
            }
        }
        teleport.update((float)dt);
    }

    @Override
    public void setTarget(final Player target){
        super.setTarget(target);
        teleport.setTarget(target);
    }

    @Override
    public void fixedUpdate(final double dt){
        super.fixedUpdate(dt);
        if (isSpawning() || isDead()){
            return;
        }
        //teleport.activate(); //todo fix; teleports outside of room
        teleport.fixedUpdate((float)dt);
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
}
