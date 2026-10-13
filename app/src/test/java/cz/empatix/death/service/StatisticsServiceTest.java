package cz.empatix.death.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class StatisticsServiceTest {

    @Test
    void accuracyIsZeroWhenNothingFired() {
        final StatisticsService s = new StatisticsService();
        assertEquals(0f, s.getAccuracy());
    }

    @Test
    void accuracyIsHitsOverFired() {
        final StatisticsService s = new StatisticsService();
        s.addBulletFired();
        s.addBulletFired();
        s.addBulletFired();
        s.addBulletFired();
        s.addBulletHit();
        s.addBulletHit();
        s.addBulletHit();
        assertEquals(0.75f, s.getAccuracy());
    }

    @Test
    void nextFloorAdvancesFloor() {
        final StatisticsService s = new StatisticsService();
        assertEquals(0, s.getFloor());
        s.nextFloor();
        s.nextFloor();
        assertEquals(2, s.getFloor());
    }

    @Test
    void killsAndItemsAreCounted() {
        final StatisticsService s = new StatisticsService();
        s.addEnemyKilled();
        s.addEnemyKilled();
        s.addItemCollected();
        assertEquals(2, s.getEnemiesKilled());
        assertEquals(1, s.getItemsCollected());
    }

    @Test
    void floorScoreFollowsFormula() {
        final StatisticsService s = new StatisticsService();
        // floor 0 -> (0 - 1) * 5 = -5
        assertEquals(-5, s.calculateScoreForFloor());
        s.nextFloor();
        s.nextFloor();
        s.nextFloor();
        // floor 3 -> (3 - 1) * 5 = 10
        assertEquals(10, s.calculateScoreForFloor());
    }

    @Test
    void enemyKillScoreMatchesFormula() {
        final StatisticsService s = new StatisticsService();
        s.nextFloor();
        s.nextFloor();
        s.nextFloor(); // floor 3
        s.addEnemyKilled();
        s.addEnemyKilled();
        s.addEnemyKilled();
        s.addEnemyKilled();
        s.addEnemyKilled(); // 5 kills
        final int expected = (int) (2.0 * Math.pow(3, 1.02) * Math.pow(5, 1.01));
        assertEquals(expected, s.calculateScoreForEnemiesKilled());
    }

    @Test
    void itemScoreMatchesFormula() {
        final StatisticsService s = new StatisticsService();
        s.nextFloor();
        s.nextFloor(); // floor 2
        s.addItemCollected();
        s.addItemCollected();
        s.addItemCollected(); // 3 items
        final int expected = (int) (4.0 * Math.pow(2, 1.02) * Math.pow(3, 1.01));
        assertEquals(expected, s.calculateScoreForItemsCollected());
    }

    @Test
    void accuracyScoreMatchesFormula() {
        final StatisticsService s = new StatisticsService();
        s.nextFloor();
        s.nextFloor();
        s.nextFloor(); // floor 3
        s.addBulletFired();
        s.addBulletFired();
        s.addBulletHit(); // accuracy 0.5
        final int expected = (int) (2.0 * Math.pow(3, 1.02) * Math.pow(0.5f * 100, 1.01));
        assertEquals(expected, s.calculateScoreForAccuracy());
    }

    @Test
    void moreKillsYieldNoLessScore() {
        final StatisticsService few = new StatisticsService();
        final StatisticsService many = new StatisticsService();
        few.nextFloor();
        many.nextFloor(); // both floor 1 so floor factor is non-zero
        few.addEnemyKilled();
        for (int i = 0; i < 10; i++) {
            many.addEnemyKilled();
        }
        assertTrue(many.calculateScoreForEnemiesKilled() >= few.calculateScoreForEnemiesKilled());
    }

    @Test
    void deeperFloorYieldsNoLessKillScore() {
        final StatisticsService shallow = new StatisticsService();
        final StatisticsService deep = new StatisticsService();
        shallow.nextFloor(); // floor 1
        for (int i = 0; i < 5; i++) {
            deep.nextFloor(); // floor 5
        }
        shallow.addEnemyKilled();
        deep.addEnemyKilled();
        assertTrue(deep.calculateScoreForEnemiesKilled() >= shallow.calculateScoreForEnemiesKilled());
    }
}
