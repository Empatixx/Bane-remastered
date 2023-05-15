package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class EnemyService {
    private final EnemyFactory enemyFactory;
    public final List<Enemy> enemies;

    public EnemyService() {
        enemies = new ArrayList<>();
        enemyFactory = new EnemyFactory();
    }

    public void update(double dt) {
        enemies.removeIf(Enemy::isDead);
        enemies.forEach(enemy -> enemy.update(dt));
    }

    public List<Drawable> toRender() {
        return List.copyOf(enemies);
    }

    public void setTarget(final Player p) {
        //enemies.forEach(enemy -> enemy.setTarget(p));
    }
    public void spawnEnemy(final Room room) {
        final Enemy e = enemyFactory.createEnemy();
        addToRoom(room, e);
    }

    public void spawnBoss(final Room room) {
        final Enemy e = enemyFactory.createBoss();
        addToRoom(room, e);
    }

    private void addToRoom(Room room, Enemy e) {
        final ThreadLocalRandom random = ThreadLocalRandom.current();

        final int tileSize = Tile.TILE_SIZE * Tile.SCALE;

        final float x = room.getX() + tileSize + random.nextInt(room.getWidth() - e.getWidth() - tileSize*2);
        final float y = room.getY() + tileSize + random.nextInt(room.getHeight() - e.getHeight() - tileSize*2);

        e.setPosition(x, y);
        enemies.add(e);
    }
}
