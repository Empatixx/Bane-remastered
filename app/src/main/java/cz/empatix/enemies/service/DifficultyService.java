package cz.empatix.enemies.service;

/**
 * Per-floor difficulty for enemy stats. Floor 0 (the first floor) is neutral (x1.0); each subsequent
 * floor adds a fixed growth fraction to health and damage. The growth rates are immutable instance
 * state supplied at construction, so future run modifiers (difficulty/"heat" modes, daily mutators,
 * per-biome tuning) can build a configured instance without touching call sites. The instance is
 * created once per run and injected down to the enemy-spawn path — not a global singleton.
 */
public class DifficultyService {

    public static final float DEFAULT_HEALTH_GROWTH_PER_FLOOR = 0.12f;
    public static final float DEFAULT_DAMAGE_GROWTH_PER_FLOOR = 0.08f;

    private final float healthGrowthPerFloor;
    private final float damageGrowthPerFloor;

    public DifficultyService() {
        this(DEFAULT_HEALTH_GROWTH_PER_FLOOR, DEFAULT_DAMAGE_GROWTH_PER_FLOOR);
    }

    public DifficultyService(final float healthGrowthPerFloor, final float damageGrowthPerFloor) {
        this.healthGrowthPerFloor = healthGrowthPerFloor;
        this.damageGrowthPerFloor = damageGrowthPerFloor;
    }

    public float healthMultiplier(final int floor) {
        return 1f + healthGrowthPerFloor * Math.max(0, floor);
    }

    public float damageMultiplier(final int floor) {
        return 1f + damageGrowthPerFloor * Math.max(0, floor);
    }

    /** Applies a multiplier to a base stat, rounding to the nearest int and never dropping below 1. */
    public int scaleStat(final int base, final float multiplier) {
        return Math.max(1, Math.round(base * multiplier));
    }
}
