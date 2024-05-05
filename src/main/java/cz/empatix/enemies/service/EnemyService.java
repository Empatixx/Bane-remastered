package cz.empatix.enemies.service;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.items.service.ItemService;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.TileImpl;
import org.dyn4j.geometry.Rectangle;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class EnemyService {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final List<Enemy> enemies = new ArrayList<>(10);
    private final EnemyDropService enemyDropService;
    private final StatisticsService statisticsService;

    public EnemyService(final ItemService itemService,
                        final StatisticsService statisticsService) {
        this.enemyDropService = new EnemyDropService(itemService);
        this.statisticsService = statisticsService;
    }

    public void update(final double dt) {
        enemies.forEach(enemy -> enemy.update(dt));
        enemies.stream()
                .filter(Enemy::canDropItems)
                .forEach(enemyDropService::tryDrop);
        enemies.removeIf(e -> {
            if (e.canRemove()){
                statisticsService.addEnemyKilled();
                return true;
            }
            return false;
        });
    }

    public void fixedUpdate(final double dt) {
        enemies.forEach(enemy -> enemy.fixedUpdate(dt));
    }

    public List<Drawable> toRender() {
        return List.copyOf(enemies);
    }

    public void setTarget(final Player p) {
        enemies.forEach(enemy -> enemy.setTarget(p));
    }

    public void spawnEnemy(final Room room) {
        final Enemy e = EnemyFactory.createEnemy(statisticsService.getFloor());

        final int tileSize = TileImpl.TILE_SIZE * TileImpl.SCALE;
        final PhysicsService physicsService = PhysicsService.getInstance();

        final PhysicsObject physicsObject = new PhysicsObject();
        physicsObject.addFixture(new Rectangle(e.getWidth(), e.getHeight()));

        do{
            final float x = room.getX() + tileSize + RANDOM.nextInt(room.getWidth() - e.getWidth() - tileSize * 2);
            final float y = room.getY() + tileSize + RANDOM.nextInt(room.getHeight() - e.getHeight() - tileSize * 2);
            physicsObject.setTranslation(x, y);
        } while (physicsService.isColliding(physicsObject));

        e.setPosition(physicsObject.getPhysicsTranslationX(), physicsObject.getPhysicsTranslationY());

        enemies.add(e);
    }

    public void spawnBoss(final Room room) {
        final Enemy e = EnemyFactory.createBoss(statisticsService.getFloor());

        final float x = room.getX() + room.getWidth() / 2f - e.getWidth() / 2f;
        final float y = room.getY() + room.getHeight() / 2f - e.getHeight() / 2f;
        e.setPosition(x, y);

        enemies.add(e);
    }

    public boolean areEnemiesDead() {
        return enemies.isEmpty();
    }

    public void cleanUp() {
        enemies.removeIf(Enemy::canRemove);

    }

    public List<Enemy> getEnemies() {
        return enemies;
    }
}
