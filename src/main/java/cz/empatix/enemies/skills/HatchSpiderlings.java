package cz.empatix.enemies.skills;

import cz.empatix.enemies.domain.Spiderling;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

public class HatchSpiderlings implements Skill{
    private final static int HATCH_DELAY = 5000;
    private final Timer timer;
    private final Supplier<Vector2f> spawnPosition;
    private final List<Spiderling> spiderlings;
    private Player target;
    public HatchSpiderlings(final Supplier<Vector2f> spawnPosition){
        this.timer = new Timer(HATCH_DELAY);
        this.spawnPosition = spawnPosition;
        spiderlings = new ArrayList<>();
    }

    public void setTarget(final Player player){
        this.target = player;
    }
    @Override
    public void fixedUpdate(final float dt){
        spiderlings.forEach(s -> s.fixedUpdate(dt));
        spiderlings.removeIf(Spiderling::canRemove);
    }

    @Override
    public void activate() {
        if(timer.isFinished()){
            timer.start();
            final Spiderling spiderling = new Spiderling();
            final Vector2f spawnPosition = this.spawnPosition.get();
            spiderling.setPosition(spawnPosition.x(), spawnPosition.y());
            spiderling.setTarget(target);
            spiderling.removeSpawning();
            spiderlings.add(spiderling);
        }
    }

    @Override
    public void update(float dt) {
        spiderlings.forEach(s -> s.update(dt));
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return List.copyOf(spiderlings);
    }
}
