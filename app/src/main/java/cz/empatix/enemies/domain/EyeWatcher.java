package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.skills.EyeWatcherProjectile;
import cz.empatix.enemies.skills.Timer;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.animation.AnimationState;

import java.util.ArrayList;
import java.util.List;
@SpawnableEnemy(minFloor = 3)
public class EyeWatcher extends BaseEnemy{
    private final static long SHOOT_TIME = 3000;
    private static final int SCALE = 3;
    private static final int WIDTH = 32 * SCALE;
    private static final int HEIGHT = 32 * SCALE;
    private static final int DEFAULT_HEALTH = 4;
    private static final int DEFAULT_DAMAGE = 1;
    private final Timer timer;
    private Player target;
    private final List<EyeWatcherProjectile> projectiles;
    public EyeWatcher() {
        super(WIDTH, HEIGHT);
        animation = Animation.builder()
                .setTotal(2, 5)
                .addState(AnimationState.IDLE, 0, 4, WIDTH, HEIGHT, 100)
                .addState(AnimationState.ATTACK, 1, 5, WIDTH, HEIGHT, 75)
                .texture(AssetsService.getTexture("ingame/eye_watcher.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = 0;
        timer = new Timer(SHOOT_TIME);
        timer.start();

        projectiles = new ArrayList<>();
    }
    @Override
    public void setTarget(final Player player){
        super.setTarget(player);
        this.target = player;
    }

    @Override
    public void update(final double dt) {
        super.update(dt);
        projectiles.forEach(projectile -> projectile.update(dt));
    }

    @Override
    public void fixedUpdate(final double dt){
        super.fixedUpdate(dt);
        projectiles.forEach(projectile -> projectile.fixedUpdate(dt));
        projectiles.removeIf(EyeWatcherProjectile::shouldRemove);
        if (isDead() || isSpawning()){
            return;
        }
        activateProjectileAttack();
    }
    private void activateProjectileAttack(){
        final AnimationState state = (AnimationState) animation.getCurrentState();
        switch (state){
            case ATTACK -> {
                if (animation.hasPlayedOnce()){
                    animation.setState(AnimationState.IDLE);
                    final EyeWatcherProjectile projectile = new EyeWatcherProjectile();
                    projectile.setPosition(physicsBody.getTranslationX() + getWidth()/2f - projectile.getWidth()/2f,
                            physicsBody.getTranslationY());
                    projectile.followTarget(target);
                    projectiles.add(projectile);
                }
            }
            default -> {
                if (timer.isFinished()){
                    animation.setState(AnimationState.ATTACK);
                    timer.start();
                }
            }
        }
    }
    @Override
    public boolean canRemove(){
        return isDead() && vitalityDisplayService.isEmpty() && animation.hasPlayedOnce() && projectiles.isEmpty();
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        state.addAll(projectiles);
        super.collectSubRenderables(state);
    }

    @Override
    public void onDeath() {
    }

    @Override
    public void onSpawn() {

    }
}
