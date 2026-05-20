package cz.empatix.enemies.skills;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.enemies.domain.GreenSpiderling;
import cz.empatix.enemies.domain.PurpleSpiderling;
import cz.empatix.enemies.domain.RedSpiderling;
import cz.empatix.player.Player;
import cz.empatix.render.domain.RenderableState;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

public class HatchSpiderlings implements Skill {
    private final static int HATCH_DELAY = 2500;
    private final Timer timer;
    private final Supplier<Vector2f> spawnPosition;
    private final List<Enemy> spiderlings;
    private Player target;
    private HatchRotation hatchRotation;

    public HatchSpiderlings(final Supplier<Vector2f> spawnPosition) {
        this.timer = new Timer(HATCH_DELAY);
        this.spawnPosition = spawnPosition;
        spiderlings = new ArrayList<>();
        hatchRotation = HatchRotation.RED;
        timer.start();
    }

    public void setTarget(final Player player) {
        this.target = player;
    }

    @Override
    public void fixedUpdate(final float dt) {
        spiderlings.forEach(s -> s.fixedUpdate(dt));
        spiderlings.removeIf(Enemy::canRemove);
    }

    @Override
    public void activate() {
        if (timer.isFinished()) {
            timer.start();
            switch (hatchRotation) {
                case RED -> {
                    final RedSpiderling spiderling = new RedSpiderling();
                    final Vector2f spawnPosition = this.spawnPosition.get();
                    spiderling.setPosition(spawnPosition.x(), spawnPosition.y());
                    spiderling.setTarget(target);
                    spiderling.removeSpawning();
                    spiderlings.add(spiderling);
                }
                case GREEN -> {
                    final GreenSpiderling spiderling = new GreenSpiderling();
                    final Vector2f spawnPosition = this.spawnPosition.get();
                    spiderling.setPosition(spawnPosition.x(), spawnPosition.y());
                    spiderling.setTarget(target);
                    spiderling.removeSpawning();
                    spiderlings.add(spiderling);
                }
                case PURPLE -> {
                    final PurpleSpiderling spiderling = new PurpleSpiderling();
                    final Vector2f spawnPosition = this.spawnPosition.get();
                    spiderling.setPosition(spawnPosition.x(), spawnPosition.y());
                    spiderling.setTarget(target);
                    spiderling.removeSpawning();
                    spiderlings.add(spiderling);
                }
            }
            hatchRotation = hatchRotation.next();
        }
    }

    public boolean canRemove() {
        return spiderlings.isEmpty();
    }

    private enum HatchRotation {
        RED,
        GREEN,
        PURPLE;

        public HatchRotation next() {
            return switch (this) {
                case RED -> GREEN;
                case GREEN -> PURPLE;
                case PURPLE -> RED;
            };
        }
    }
    public void despawnAll(){
        spiderlings.forEach(Enemy::despawn);
    }

    @Override
    public void update(float dt) {
        spiderlings.forEach(s -> s.update(dt));
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        state.addAll(spiderlings);
    }
}
