package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.*;
import cz.empatix.enemies.skills.HatchSpiderlings;
import cz.empatix.enemies.skills.SpitCobweb;
import cz.empatix.enemies.skills.Timer;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;

@SpawnableEnemy(type = EnemyType.BOSS, maxFloor = 1, minFloor = 1)
public class Spider extends BossBaseEnemy {
    private static final int SCALE = 3;
    private static final int HEIGHT = 40 * SCALE;
    private static final int WIDTH = 64 * SCALE;
    private static final int DEFAULT_DAMAGE = 2;
    private static final int DEFAULT_HEALTH = 75;
    private static final int MOVE_FORCE = 32_500_000;
    private final HatchSpiderlings hatchSpiderlings;
    private final SpitCobweb spitCobweb;

    public Spider() {
        super(WIDTH, HEIGHT, "Spider king");
        animation = Animation.builder()
                .setTotal(3, 5)
                .addState(AnimationState.IDLE, 0, 5, WIDTH, HEIGHT, 120)
                .addState(AnimationState.SPELL0, 1, 3, WIDTH, HEIGHT, 150)
                .addState(AnimationState.DEATH, 2, 3, WIDTH, HEIGHT, 150)
                .texture(AssetsService.getTexture("ingame/spider.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
        hatchSpiderlings = new HatchSpiderlings(() -> new Vector2f(getX() + getWidth() / 2, getY() + getHeight() / 2));
        spitCobweb = new SpitCobweb(animation, physicsBody, () -> {
            if (aStarTargetting.isTargetLeftFromEnemy()){
                return new Vector2f(getX() + getWidth() / 2 - 12 * SCALE, getY() + getHeight() - 5 * SCALE);
            } else {
                return new Vector2f(getX() + getWidth() / 2 + 12 * SCALE, getY() + getHeight() - 5 * SCALE);
            }
        });
    }

    @Override
    public void update(double dt) {
        super.update(dt);
        hatchSpiderlings.update((float)dt);
        spitCobweb.update((float)dt);
    }
    @Override
    public void setTarget(final Player player){
        super.setTarget(player);
        hatchSpiderlings.setTarget(player);
        spitCobweb.setTarget(player);
    }

    @Override
    public void fixedUpdate(double dt) {
        super.fixedUpdate(dt);

        hatchSpiderlings.fixedUpdate((float)dt);
        spitCobweb.fixedUpdate((float)dt);
        if (isDead()){
            return;
        }

        if (animation.getCurrentState() == AnimationState.SPELL0){
            if (animation.hasPlayedOnce()){
                animation.setState(AnimationState.IDLE);
            }
        }
        if (animation.getCurrentState() == AnimationState.SPELL0){
            physicsBody.setVelocity(0,0);
            physicsBody.clearForce();
            physicsBody.clearAccumulatedForce();
            return;
        }
        hatchSpiderlings.activate();
        spitCobweb.activate();
    }
    @Override
    public List<Renderable> getSubRenderables() {
        final List<Renderable> renderables = new ArrayList<>(super.getSubRenderables());
        renderables.addAll(hatchSpiderlings.getSubRenderables());
        renderables.addAll(spitCobweb.getSubRenderables());
        return renderables;
    }
    @Override
    public void onDeath() {
        animation.setState(AnimationState.DEATH);
        hatchSpiderlings.despawnAll();
        spitCobweb.destroy();
    }

    @Override
    public void onSpawn() {

    }
    @Override
    public boolean canRemove() {
        return isDead() && vitalityDisplayService.isEmpty() && animation.hasPlayedOnce() && hatchSpiderlings.canRemove();
    }

}
