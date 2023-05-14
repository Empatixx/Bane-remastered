package cz.empatix.enemies.domain;

public interface Enemy {
    void update(double dt);
    void onDeath();
    void onHit();
    void onCollision();
    void onSpawn();
}
