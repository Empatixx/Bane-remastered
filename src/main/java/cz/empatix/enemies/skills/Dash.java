package cz.empatix.enemies.skills;

public class Dash implements Skill {
    private final static int DASH_COOLDOWN = 10000;
    private final static int DASH_DURATION = 2000;
    private final static int ADDITIONAL_FORCE = 1000;
    private final Runnable activate;
    private final Runnable deactivate;
    private final Timer cooldown;
    private final Timer duration;
    private boolean active = false;

    public Dash(final Runnable activate,
                final Runnable deactivate) {
        this.activate = activate;
        this.deactivate = deactivate;
        cooldown = new Timer(DASH_COOLDOWN);
        duration = new Timer(DASH_DURATION);
    }
    public void activate() {
        if (cooldown.isFinished()) {
            cooldown.start();
            duration.start();
            cooldown.start();
            activate.run();
            active = true;
        }
    }

    public void update(final float dt) {
        if(duration.isFinished() && active) {
            deactivate.run();
            active = false;
        }
    }

}
