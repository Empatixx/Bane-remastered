package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderableState;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

public class KingSlimeArmageddon implements Skill {
    private final static int SHOOT_COOLDOWN = 150;
    private static final float INACCURACY_INCREASE = 0.25f;
    private final List<BaseSlimebullet> slimebullets = new ArrayList<>(100);
    private float currentInaccuracy = 0;
    private final Supplier<Vector2f> positionSupplier;
    private final Supplier<BaseSlimebullet> bulletSupplier;
    private final Timer cooldown;

    public KingSlimeArmageddon(final Supplier<Vector2f> positionSupplier,
                               final Supplier<BaseSlimebullet> bulletSupplier) {
        this.bulletSupplier = bulletSupplier;
        this.positionSupplier = positionSupplier;
        cooldown = new Timer(SHOOT_COOLDOWN);
    }

    @Override
    public void activate() {
        if (!cooldown.isFinished()) {
            return;
        }
        currentInaccuracy += INACCURACY_INCREASE;

        cooldown.start();
        final Vector2f position = positionSupplier.get();
        final BaseSlimebullet slimebullet = bulletSupplier.get();
        slimebullet.setPosition(position.x, position.y);
        slimebullet.setDirection(0, 0, 0, 0, currentInaccuracy);
        slimebullets.add(slimebullet);
    }

    @Override
    public void update(float dt) {
        slimebullets.forEach(slimebullet -> slimebullet.update(dt));
        slimebullets.removeIf(BaseSlimebullet::shouldRemove);
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        state.addAll(slimebullets);
    }
    public boolean noBullets(){
        return slimebullets.isEmpty();
    }

    public void clear() {
        slimebullets.forEach(BaseSlimebullet::destroy);
        slimebullets.clear();
    }
}
