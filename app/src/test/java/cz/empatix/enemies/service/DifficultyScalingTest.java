package cz.empatix.enemies.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Pure coverage of the per-floor difficulty multipliers and integer stat scaling. No GL, no harness.
 */
class DifficultyScalingTest {

    @Test
    void floorZeroIsNeutral() {
        assertEquals(1.0f, DifficultyScaling.healthMultiplier(0), 1e-6);
        assertEquals(1.0f, DifficultyScaling.damageMultiplier(0), 1e-6);
    }

    @Test
    void multipliersGrowLinearlyPerFloor() {
        assertEquals(1.6f, DifficultyScaling.healthMultiplier(5), 1e-4); // 1 + 0.12*5
        assertEquals(1.4f, DifficultyScaling.damageMultiplier(5), 1e-4); // 1 + 0.08*5
    }

    @Test
    void negativeFloorsAreClampedToNeutral() {
        assertEquals(1.0f, DifficultyScaling.healthMultiplier(-3), 1e-6);
        assertEquals(1.0f, DifficultyScaling.damageMultiplier(-3), 1e-6);
    }

    @Test
    void scaleStatRoundsToNearestIntAndNeverDropsBelowOne() {
        assertEquals(16, DifficultyScaling.scaleStat(10, 1.6f));
        assertEquals(7, DifficultyScaling.scaleStat(5, 1.4f));
        assertEquals(1, DifficultyScaling.scaleStat(1, 0.1f), "a stat never scales below 1");
    }
}
