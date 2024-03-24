package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.skills.Timer;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;

public class EyeWatcher extends BaseEnemy{
    private final static long SHOOT_TIME = 2000;
    private static final int SCALE = 3;
    private static final int WIDTH = 32 * SCALE;
    private static final int HEIGHT = 32 * SCALE;
    private static final int DEFAULT_HEALTH = 4;
    private static final int DEFAULT_DAMAGE = 1;
    private final Timer timer;
    private Player target;
    public EyeWatcher() {
        super(WIDTH, HEIGHT);
        animation = Animation.builder()
                .setTotal(2, 5)
                .addState(AnimationState.IDLE, 0, 4, WIDTH, HEIGHT, 100)
                .addState(AnimationState.ATTACK, 1, 5, WIDTH, HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/eye_watcher.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = 0;
        timer = new Timer(SHOOT_TIME);
        timer.start();
    }
    @Override
    public void setTarget(final Player player){
        this.target = player;
    }
    @Override
    public void fixedUpdate(final double dt){
        super.fixedUpdate(dt);
        if (timer.isFinished()){
            timer.start();
        }
    }

    @Override
    public void onDeath() {
    }

    @Override
    public void onSpawn() {

    }
}
