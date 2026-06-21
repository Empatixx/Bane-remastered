package cz.empatix.enemies.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Pure coverage of the per-floor difficulty multipliers and integer stat scaling. No GL, no harness;
 * the service is a plain injectable instance (no singleton), so it is constructed directly per test.
 */
class DifficultyServiceTest {

    private final DifficultyService difficulty = new DifficultyService();

    @Test
    void floorZeroIsNeutral() {
        assertEquals(1.0f, difficulty.healthMultiplier(0), 1e-6);
        assertEquals(1.0f, difficulty.damageMultiplier(0), 1e-6);
    }

    @Test
    void multipliersGrowLinearlyPerFloor() {
        assertEquals(1.6f, difficulty.healthMultiplier(5), 1e-4); // 1 + 0.12*5
        assertEquals(1.4f, difficulty.damageMultiplier(5), 1e-4); // 1 + 0.08*5
    }

    @Test
    void negativeFloorsAreClampedToNeutral() {
        assertEquals(1.0f, difficulty.healthMultiplier(-3), 1e-6);
        assertEquals(1.0f, difficulty.damageMultiplier(-3), 1e-6);
    }

    @Test
    void scaleStatRoundsToNearestIntAndNeverDropsBelowOne() {
        assertEquals(16, difficulty.scaleStat(10, 1.6f));
        assertEquals(7, difficulty.scaleStat(5, 1.4f));
        assertEquals(1, difficulty.scaleStat(1, 0.1f), "a stat never scales below 1");
    }

    @Test
    void customGrowthRatesAreHonoured() {
        final DifficultyService custom = new DifficultyService(0.5f, 0.25f);

        assertEquals(2.0f, custom.healthMultiplier(2), 1e-4); // 1 + 0.5*2
        assertEquals(1.5f, custom.damageMultiplier(2), 1e-4); // 1 + 0.25*2
    }
}
