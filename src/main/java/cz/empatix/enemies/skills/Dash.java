package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Renderable;

import java.util.List;
import java.util.function.Consumer;

public class Dash implements Skill {
    private final static int DASH_COOLDOWN = 3000;
    private final static int DASH_DURATION = 1000;
    private final static int ADDITIONAL_FORCE = 30000000;

    private final Runnable activate;
    private final Runnable deactivate;
    private final Consumer<Integer> forceConsumer;
    private final Timer cooldown;
    private final Timer duration;
    private boolean active = false;

    public Dash(final Runnable activate,
                final Runnable deactivate,
                final Consumer<Integer> forceConsumer) {
        this.activate = activate;
        this.deactivate = deactivate;
        this.forceConsumer = forceConsumer;
        cooldown = new Timer(DASH_COOLDOWN);
        duration = new Timer(DASH_DURATION);
    }
    public void activate() {
        if (cooldown.isFinished()) {
            forceConsumer.accept(ADDITIONAL_FORCE);
            cooldown.start();
            duration.start();
            cooldown.start();
            activate.run();
            active = true;
        }
    }

    public void update(final float dt) {
        if(duration.isFinished() && active) {
            forceConsumer.accept(-ADDITIONAL_FORCE);
            deactivate.run();
            active = false;
        }
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return List.of();
    }


}
