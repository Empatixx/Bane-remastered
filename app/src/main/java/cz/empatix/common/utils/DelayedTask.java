package cz.empatix.common.utils;

public class DelayedTask implements Task {
    private final float deltaDelay;
    private final Runnable task;
    private float timeElapsed = 0;
    private boolean isDone = false;
    private boolean isRunning = false;
    public DelayedTask(final float deltaDelay, final Runnable task) {
        this.deltaDelay = deltaDelay;
        this.task = task;
    }

    @Override
    public void update(final float dt) {
        if (isDone || !isRunning) {
            return;
        }
        timeElapsed += dt;
        if (timeElapsed >= deltaDelay) {
            task.run();
            isDone = true;
        }
    }

    @Override
    public void start() {
        isRunning = true;
        reset();
    }

    @Override
    public void reset() {
        timeElapsed = 0;
        isDone = false;
    }

    @Override
    public void stop() {
        isRunning = false;
    }
}
