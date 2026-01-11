package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderableState;
import java.util.function.Consumer;

public class Dash implements Skill {
    private final static int DASH_COOLDOWN = 3000;
    private final static int DASH_DURATION = 1000;
    private final static int ADDITIONAL_FORCE = 12_500_000;

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
    @Override
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

    @Override
    public void update(final float dt) {
        if(duration.isFinished() && active) {
            forceConsumer.accept(-ADDITIONAL_FORCE);
            deactivate.run();
            active = false;
        }
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
    }


}
