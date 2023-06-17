package cz.empatix.rooms.service;

import cz.empatix.enemies.service.EnemyService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.service.RoomLockService;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.List;

public class BossRoom implements Room {
    private final EnemyService enemyService;
    private final RoomLockService roomLockService;
    private final List<RoomDirection> neighbors;
    private final TileMap tileMap;
    private boolean visited;
    public BossRoom(final int rows,final int cols,final int xOffset,final int yOffset,
                    final List<RoomDirection> roomDirections) {
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, roomDirections);
        roomLockService = new RoomLockService(this, roomDirections);
        neighbors = roomDirections;
        enemyService = new EnemyService();
        visited = false;
    }
    @Override
    public void enterEvent(final Player p) {
        if(!visited){
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
        if(!enemyService.areEnemiesAlive() && !roomLockService.isLocked()){
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
}
