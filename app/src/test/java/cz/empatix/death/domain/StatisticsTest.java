package cz.empatix.death.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class StatisticsTest {

    @Test
    void freshStatisticsAreAllZero() {
        final Statistics s = new Statistics();
        assertEquals(0, s.getBulletHit());
        assertEquals(0, s.getBulletFired());
        assertEquals(0, s.getEnemyKilled());
        assertEquals(0, s.getItemsCollected());
        assertEquals(0, s.getFloor());
    }

    @Test
    void addersIncrementIndependentCounters() {
        final Statistics s = new Statistics();
        s.addBulletHit();
        s.addBulletHit();
        s.addBulletFired();
        s.addEnemyKilled();
        s.addEnemyKilled();
        s.addEnemyKilled();
        s.addItemCollected();
        s.addFloor();
        s.addFloor();

        assertEquals(2, s.getBulletHit());
        assertEquals(1, s.getBulletFired());
        assertEquals(3, s.getEnemyKilled());
        assertEquals(1, s.getItemsCollected());
        assertEquals(2, s.getFloor());
    }
}
