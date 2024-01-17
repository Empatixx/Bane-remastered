package cz.empatix.death.domain;

public class Statistics {
    private int bulletHit;
    private int bulletFired;
    private int enemyKilled;
    private int itemsCollected;
    private int floor;
    public Statistics(){
        bulletHit = 0;
        bulletFired = 0;
        enemyKilled = 0;
        itemsCollected = 0;
        floor = 0;
    }
    public void addBulletHit(){
        bulletHit++;
    }
    public void addBulletFired(){
        bulletFired++;
    }
    public void addEnemyKilled(){
        enemyKilled++;
    }
    public void addItemCollected(){
        itemsCollected++;
    }

    public int getBulletFired() {
        return bulletFired;
    }

    public int getBulletHit() {
        return bulletHit;
    }

    public int getEnemyKilled() {
        return enemyKilled;
    }

    public int getFloor() {
        return floor;
    }

    public int getItemsCollected() {
        return itemsCollected;
    }

    public void addFloor() {
        floor++;
    }
}
