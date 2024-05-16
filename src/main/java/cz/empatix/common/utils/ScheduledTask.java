package cz.empatix.common.utils;

public class ScheduledTask implements Task{
    private final float deltaDelay;
    private final Runnable task;
    private float timeElapsed = 0;
    private boolean isRunning = false;
    public ScheduledTask(final float deltaDelay, final Runnable task) {
        this.deltaDelay = deltaDelay;
        this.task = task;
    }

    @Override
    public void update(final float dt) {
        if (!isRunning) {
            return;
        }
        timeElapsed += dt;
        if (timeElapsed >= deltaDelay) {
            task.run();
            timeElapsed = 0;
        }
    }

    @Override
    public void start() {
        isRunning = true;
    }

    @Override
    public void reset() {
        timeElapsed = 0;
    }

    @Override
    public void stop() {
        isRunning = false;
    }
}
