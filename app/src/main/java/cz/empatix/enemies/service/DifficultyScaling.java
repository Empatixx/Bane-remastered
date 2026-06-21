package cz.empatix.enemies.service;

/**
 * Per-floor difficulty multipliers for enemy stats. Floor 0 (the first floor) is neutral (x1.0);
 * each subsequent floor adds a fixed fraction. Pure and GL-free so it is trivially unit-tested and
 * easy to retune in one place.
 */
public final class DifficultyScaling {

    public static final float HEALTH_GROWTH_PER_FLOOR = 0.12f;
    public static final float DAMAGE_GROWTH_PER_FLOOR = 0.08f;

    private DifficultyScaling() {
    }

    public static float healthMultiplier(final int floor) {
        return 1f + HEALTH_GROWTH_PER_FLOOR * Math.max(0, floor);
    }

    public static float damageMultiplier(final int floor) {
        return 1f + DAMAGE_GROWTH_PER_FLOOR * Math.max(0, floor);
    }

    /** Applies a multiplier to a base stat, rounding to the nearest int and never dropping below 1. */
    public static int scaleStat(final int base, final float multiplier) {
        return Math.max(1, Math.round(base * multiplier));
    }
}
