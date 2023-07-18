package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.items.service.ItemService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class EnemyService {
    private final EnemyFactory enemyFactory = new EnemyFactory();
    private final List<Enemy> enemies = new ArrayList<>(10);
    private final EnemyDropService enemyDropService;
    public EnemyService(final ItemService itemService) {
        this.enemyDropService = new EnemyDropService(itemService);
    }

    public void update(final double dt) {
        enemies.forEach(enemy -> enemy.update(dt));
        enemies.stream()
                .filter(Enemy::canDropItems)
                .forEach(enemyDropService::tryDrop);
        enemies.removeIf(Enemy::canRemove);
    }

    public void fixedUpdate(final double dt) {
        enemies.forEach(enemy -> enemy.fixedUpdate(dt));
    }

    public List<Drawable> toRender() {
        return List.copyOf(enemies);
    }

    public void setTarget(final Player p) {
        //enemies.forEach(enemy -> enemy.setTarget(p));
    }

    public void spawnEnemy(final Room room) {
        final Enemy e = enemyFactory.createEnemy();

        final ThreadLocalRandom random = ThreadLocalRandom.current();
        final int tileSize = Tile.TILE_SIZE * Tile.SCALE;

        final float x = room.getX() + tileSize + random.nextInt(room.getWidth() - e.getWidth() - tileSize * 2);
        final float y = room.getY() + tileSize + random.nextInt(room.getHeight() - e.getHeight() - tileSize * 2);
        e.setPosition(x, y);

        enemies.add(e);
    }

    public void spawnBoss(final Room room) {
        final Enemy e = enemyFactory.createBoss();

        final float x = room.getX() + room.getWidth() / 2f - e.getWidth() / 2f;
        final float y = room.getY() + room.getHeight() / 2f - e.getHeight() / 2f;
        e.setPosition(x, y);

        enemies.add(e);
    }

    public boolean areEnemiesDead() {
        return enemies.size() <= 0;
    }

}
