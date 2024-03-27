package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.domain.*;
import cz.empatix.enemies.skills.Timer;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;

import java.util.ArrayList;
import java.util.List;

@SpawnableEnemy(type = EnemyType.BOSS)
public class Spider extends BossBaseEnemy {
    private static final int SCALE = 3;
    private static final int HEIGHT = 40 * SCALE;
    private static final int WIDTH = 64 * SCALE;
    private static final int DEFAULT_DAMAGE = 1;
    private static final int DEFAULT_HEALTH = 7;
    private static final int MOVE_FORCE = 32_500_000;
    private List<Spiderling> spiderlings;
    private Timer timer;
    private Player target;

    public Spider() {
        super(WIDTH, HEIGHT);
        animation = Animation.builder()
                .setTotal(1, 5)
                .addState(AnimationState.IDLE, 0, 5, WIDTH, HEIGHT, 120)
                .texture(AssetsService.getTexture("ingame/spider.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
        spiderlings = new ArrayList<>();
        timer = new Timer(5000);
    }

    @Override
    public void update(double dt) {
        super.update(dt);
        if (timer.isFinished()) {
            timer.start();
            final Spiderling spiderling = new Spiderling();
            spiderling.setPosition(getX() + getWidth() / 2 - spiderling.getWidth() / 2, getY() + getHeight() / 2 - spiderling.getHeight() / 2);
            spiderling.setTarget(target);
            spiderlings.add(spiderling);
        }
        spiderlings.forEach(s -> s.update(dt));
    }
    @Override
    public void setTarget(final Player player){
        super.setTarget(player);
        this.target = player;
    }

    @Override
    public void fixedUpdate(double dt) {
        super.fixedUpdate(dt);
        spiderlings.forEach(s -> s.fixedUpdate(dt));
        spiderlings.removeIf(BaseEnemy::canRemove);
    }
    @Override
    public List<Renderable> getSubRenderables() {
        final List<Renderable> renderables = new ArrayList<>(super.getSubRenderables());
        renderables.addAll(spiderlings);
        return renderables;
    }
    @Override
    public void onDeath() {

    }

    @Override
    public void onSpawn() {

    }
}
