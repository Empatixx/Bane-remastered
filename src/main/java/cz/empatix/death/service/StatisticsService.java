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
        return (int) (2.0 * Math.pow(1.02, getFloor()) * Math.pow(1.01, getAccuracy() * 100));
    }

    public int calculateScoreForEnemiesKilled() {
        return (int) (2.0 * Math.pow(1.02, getFloor()) * Math.pow(1.01, statistics.getEnemyKilled()));
    }

    public int calculateScoreForItemsCollected() {
        return (int) (4.0 * Math.pow(1.02, getFloor()) * Math.pow(1.01, statistics.getItemsCollected()));
    }

    public int calculateScoreForFloor() {
        return getFloor() * 5;
    }

    public void nextFloor() {
        statistics.addFloor();
    }
}
