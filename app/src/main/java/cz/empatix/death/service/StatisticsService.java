package cz.empatix.death.service;

import cz.empatix.death.domain.Statistics;

public class StatisticsService {
    private final Statistics statistics;

    public StatisticsService() {
        statistics = new Statistics();
    }

    public void addBulletHit() {
        statistics.addBulletHit();
    }

    public void addBulletFired() {
        statistics.addBulletFired();
    }

    public void addEnemyKilled() {
        statistics.addEnemyKilled();
    }

    public void addItemCollected() {
        statistics.addItemCollected();
    }

    public float getAccuracy() {
        if (statistics.getBulletFired() == 0) {
            return 0;
        }
        return (float) statistics.getBulletHit() / (float) statistics.getBulletFired();
    }

    public int getFloor() {
        return statistics.getFloor();
    }

    public int calculateScoreForAccuracy() {
        return (int) (2.0 * Math.pow(getFloor(), 1.02) * Math.pow(getAccuracy() * 100, 1.01));
    }

    public int calculateScoreForEnemiesKilled() {
        return (int) (2.0 * Math.pow(getFloor(), 1.02) * Math.pow(statistics.getEnemyKilled(), 1.01));
    }

    public int calculateScoreForItemsCollected() {
        return (int) (4.0 * Math.pow(getFloor(), 1.02) * Math.pow(statistics.getItemsCollected(), 1.01));
    }

    public int calculateScoreForFloor() {
        return (getFloor() - 1) * 5;
    }

    public void nextFloor() {
        statistics.addFloor();
    }

    public int getEnemiesKilled() {
        return statistics.getEnemyKilled();
    }

    public int getItemsCollected() {
        return statistics.getItemsCollected();
    }
}
