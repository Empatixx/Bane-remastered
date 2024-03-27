package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.skills.BaseSlimebullet;
import cz.empatix.enemies.skills.RedSlimebullet;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;

import java.util.ArrayList;
import java.util.List;
@SpawnableEnemy
public class RedSlime extends BaseEnemy {
    private final static int MOVE_FORCE = 2_500_000;
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int) (64 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (48 * SCALE);
    private final static int DEFAULT_HEALTH = 9;
    private final static int DEFAULT_DAMAGE = 1;
    private int totalHitCount = 0;
    private final List<RedSlimebullet> slimebullets = new ArrayList<>();

    public RedSlime() {
        super(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        animation = Animation.builder()
                .setTotal(2, 6)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.DEATH, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .texture(AssetsService.getTexture("ingame/redslime.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;

    }

    @Override
    public void update(double dt) {
        super.update(dt);
        slimebullets.forEach(s -> s.update(dt));
    }



    @Override
    public void fixedUpdate(double dt) {
        super.fixedUpdate(dt);
        slimebullets.forEach(s -> s.fixedUpdate(dt));
        slimebullets.removeIf(BaseSlimebullet::shouldRemove);
    }

    @Override
    public void hit(final int damage) {
        totalHitCount++;
        super.hit(damage);
    }

    @Override
    public void onDeath() {
        animation.setState(AnimationState.DEATH);
        for (int i = 0; i < totalHitCount; i++){
            final RedSlimebullet slimebullet = new RedSlimebullet();
            slimebullet.setPosition(getX() + getWidth()/2f, getY() + getHeight()/2f);
            slimebullet.setDirection(0,0,0,0, i * 1.00f);
            slimebullets.add(slimebullet);
        }
    }
    @Override
    public boolean canRemove() {
        return isDead() && animation.hasPlayedOnce() && slimebullets.isEmpty();
    }


    @Override
    public List<Renderable> getSubRenderables() {
        final List<Renderable> parentRenderables = super.getSubRenderables();
        final List<Renderable> renderables = new ArrayList<>(parentRenderables.size() + slimebullets.size());
        renderables.addAll(parentRenderables);
        renderables.addAll(slimebullets);
        return renderables;
    }

    @Override
    public void onSpawn() {

    }
}
