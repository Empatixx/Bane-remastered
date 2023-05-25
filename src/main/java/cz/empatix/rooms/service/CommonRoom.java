package cz.empatix.rooms.service;

import cz.empatix.enemies.service.EnemyService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.List;

public class CommonRoom implements Room {
    private static final int MAX_ENEMIES = 5;
    private final EnemyService enemyService;
    private final RoomObjectService roomObjectService;
    private final List<RoomDirection> neighbors;
    private final TileMap tileMap;
    private boolean visited;

    public CommonRoom(final int rows, final int cols, final int xOffset, final int yOffset,
                      final List<RoomDirection> roomDirections) {
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, roomDirections);
        neighbors = roomDirections;
        enemyService = new EnemyService();
        roomObjectService = new RoomObjectService();
        visited = false;
    }

    @Override
    public void enterEvent(final Player p) {
        if (!visited) {
            visited = true;
            for (int i = 0; i < MAX_ENEMIES; i++){
                enemyService.spawnEnemy(this);
            }
            enemyService.setTarget(p);
        }
    }

    @Override
    public void leaveEvent(final Player p) {
    }

    @Override
    public void init() {
    }

    @Override
    public void update(double dt) {
        enemyService.update(dt);
    }

    @Override
    public List<Renderable> toRender() {
        final Renderable tiles = (DrawableBatch) tileMap::toRender;
        final Renderable enemies = (DrawableBatch) enemyService::toRender;
        final Renderable roomObjects = (DrawableBatch) roomObjectService::toRender;
        return List.of(tiles, enemies, roomObjects);
    }

    @Override
    public int getWidth() {
        return tileMap.getWidth();
    }

    @Override
    public int getHeight() {
        return tileMap.getHeight();
    }
    @Override
    public int getX() {
        return tileMap.getX();
    }

    @Override
    public int getY() {
        return tileMap.getY();
    }
}
