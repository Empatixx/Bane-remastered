package cz.empatix.rooms.service;

import cz.empatix.enemies.service.EnemyService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.service.RoomLockService;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.*;

public class BossRoom implements Room {
    private final EnemyService enemyService;
    private final RoomLockService roomLockService;
    private final Map<RoomDirection, Room> neighbors;
    private final TileMap tileMap;
    private boolean visited;

    public BossRoom(final int rows, final int cols, final int xOffset, final int yOffset,
                    final Set<RoomDirection> neighbors) {
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, neighbors);
        roomLockService = new RoomLockService(this, neighbors);
        enemyService = new EnemyService();
        visited = false;
        this.neighbors = new HashMap<>(neighbors.size());
    }

    @Override
    public void enterEvent(final Player p) {
        if (!visited) {
            enemyService.spawnBoss(this);
            visited = true;
            roomLockService.setState(true);
        }
        enemyService.setTarget(p);

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
        roomLockService.update(dt);
        if (!enemyService.areEnemiesAlive() && !roomLockService.isLocked()) {
            roomLockService.setState(false);
        }
    }

    @Override
    public void fixedUpdate(double dt) {
        enemyService.fixedUpdate(dt);
    }

    @Override
    public List<Renderable> toRender() {
        final Renderable tiles = (DrawableBatch) tileMap::toRender;
        final Renderable enemies = (DrawableBatch) enemyService::toRender;
        final Renderable gateWalls = (DrawableBatch) roomLockService::toRender;
        return List.of(tiles, enemies, gateWalls);
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

    @Override
    public void addNeighbour(RoomDirection direction, Room room) {
        neighbors.put(direction, room);
    }

    @Override
    public Map<RoomDirection, Room> getNeighboursMap() {
        return neighbors;
    }
}
