package cz.empatix.enemies.skills;

import cz.empatix.render.domain.RenderableState;

import java.util.function.Consumer;

public class Healing implements Skill{
    private final static int HEALING_COOLDOWN = 30000;
    private final static int DURATION = 5000;
    private final static int TOTAL_HEAL_TICKS = 5;
    private final Timer cooldown;
    private final Timer tickTimer;
    private final int healAmount;
    private final Consumer<Integer> healCallback;
    private int currentHealTick;
    public Healing(final int healAmount,
                   final Consumer<Integer> healCallback) {
        this.healAmount = healAmount;
        this.healCallback = healCallback;
        this.cooldown = new Timer(HEALING_COOLDOWN);
        this.tickTimer = new Timer(DURATION / TOTAL_HEAL_TICKS);
        this.currentHealTick = TOTAL_HEAL_TICKS;

        cooldown.start();
    }
    @Override
    public void activate() {
        if (!cooldown.isFinished()) {
            return;
        }
        currentHealTick = 0;
        tickTimer.start();
        cooldown.start();
        healCallback.accept(healAmount);
    }

    @Override
    public void update(float dt) {
        if (tickTimer.isFinished() && currentHealTick < TOTAL_HEAL_TICKS) {
            healCallback.accept(healAmount);
            tickTimer.start();
            currentHealTick++;
        }
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        // No sub-renderables
    }

    public boolean isActive() {
        return currentHealTick < TOTAL_HEAL_TICKS;
    }
}
