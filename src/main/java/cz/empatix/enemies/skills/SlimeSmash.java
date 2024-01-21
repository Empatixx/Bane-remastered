package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

public class SlimeSmash implements Skill {
    private final static int SMASH_COOLDOWN = 2000;
    private final static int DEFAULT_TOTAL_BULLETS_PER_SMASH = 5;
    private final int totalBulletsPerSmash;
    private final List<BaseSlimebullet> bullets;
    private final Supplier<Vector2f> shooterPositionSupplier;
    private final Supplier<BaseSlimebullet> bulletSupplier;
    private final Timer cooldown;
    public SlimeSmash(final Supplier<Vector2f> shooterPositionSupplier,
                      final Supplier<BaseSlimebullet> bulletSupplier) {
        this.shooterPositionSupplier = shooterPositionSupplier;
        this.totalBulletsPerSmash = DEFAULT_TOTAL_BULLETS_PER_SMASH;
        this.bullets = new ArrayList<>(totalBulletsPerSmash * 3);
        this.cooldown = new Timer(SMASH_COOLDOWN);
        this.bulletSupplier = bulletSupplier;
    }
    public SlimeSmash(final Supplier<Vector2f> shooterPositionSupplier,
                      final int totalBulletsPerSmash,
                      final Supplier<BaseSlimebullet> bulletSupplier) {
        this.shooterPositionSupplier = shooterPositionSupplier;
        this.totalBulletsPerSmash = totalBulletsPerSmash;
        this.bullets = new ArrayList<>(totalBulletsPerSmash * 3);
        this.cooldown = new Timer(SMASH_COOLDOWN);
        this.bulletSupplier = bulletSupplier;

    }
    @Override
    public void activate() {
        if (!cooldown.isFinished()) {
            return;
        }
        cooldown.start();
        final Vector2f shooterPosition = shooterPositionSupplier.get();
        for (int i = 0; i < totalBulletsPerSmash; i++) {
            final BaseSlimebullet bullet = bulletSupplier.get();
            bullet.setPosition(shooterPosition.x(), shooterPosition.y());
            bullet.setDirection(shooterPosition.x(), shooterPosition.y(), shooterPosition.x(), shooterPosition.y(), (float)(Math.PI * 2 * (i+1)) / totalBulletsPerSmash);
            bullets.add(bullet);
        }
    }

    @Override
    public void update(float dt) {
        bullets.forEach(bullet -> bullet.update(dt));
        bullets.removeIf(BaseSlimebullet::shouldRemove);
    }
    public boolean noBullets() {
        return bullets.isEmpty();
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return List.copyOf(bullets);
    }

    public void clear() {
        bullets.forEach(BaseSlimebullet::destroy);
        bullets.clear();
    }
}
