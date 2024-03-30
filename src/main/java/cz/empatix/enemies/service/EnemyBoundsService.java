package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;

import java.util.HashMap;
import java.util.Map;

public class EnemyBoundsService {
    private Map<Enemy, Vector2f> previousPositions = new HashMap<>();
    private final Room room;
    private final EnemyService enemyService;
    public EnemyBoundsService(final Room room,
                              final EnemyService enemyService){
        this.room = room;
        this.enemyService = enemyService;
    }
    public void fixedUpdate(final double dt){
        enemyService.getEnemies().forEach(e -> {
            final float roomX = room.getX();
            final float roomY = room.getY();
            final float roomWidth = room.getWidth();
            final float roomHeight = room.getHeight();

            final Vector2f now = e.getPosition();
            final Vector2f previous = previousPositions.get(e);

            if (previous == null){
                previousPositions.put(e, new Vector2f(now));
                return;
            }
            if (now.x < roomX || now.x + e.getWidth() > roomX + roomWidth){
                e.setPosition(previous.x, now.y);
            }
            if (now.y < roomY || now.y + e.getHeight() > roomY + roomHeight){
                e.setPosition(now.x, previous.y);
            }
            previousPositions.put(e, new Vector2f(now));
        });
    }
}
