package cz.empatix.enemies.skills;

public class Timer {
    private long lastTime;
    private final long totalTime;
    public Timer(final long totalTime) {
        this.totalTime = totalTime;
    }
    public boolean isFinished() {
        return System.currentTimeMillis() - lastTime > totalTime;
    }
    public void start() {
        lastTime = System.currentTimeMillis();
    }
}
