package cz.empatix.rooms.service;

import cz.empatix.enemies.service.EnemyService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.map.service.MapLoaderService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.domain.Ladder;
import cz.empatix.roomobjects.service.RoomLockService;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class BossRoom implements Room {
    private final MapLoaderService mapLoaderService;
    private final ItemService itemService;
    private final EnemyService enemyService;
    private final RoomObjectService roomObjectService;
    private final RoomLockService roomLockService;
    private final Map<RoomDirection, Room> neighbors;
    private final TileMap tileMap;
    private boolean visited = false;

    public BossRoom(final ItemService itemService,
                    final MapLoaderService mapLoaderService,
                    final int rows, final int cols, final int xOffset, final int yOffset,
                    final Set<RoomDirection> neighbors) {
        this.itemService = itemService;
        this.mapLoaderService = mapLoaderService;
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, neighbors);
        roomLockService = new RoomLockService(this, neighbors);
        enemyService = new EnemyService(itemService);
        roomObjectService = new RoomObjectService();
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
        roomObjectService.update(dt);
        if (enemyService.areEnemiesDead() && roomLockService.isLocked()) {
            roomLockService.setState(false);

            final Ladder ladder = new Ladder(mapLoaderService, itemService);
            ladder.setPosition(tileMap.getX() + tileMap.getWidth() / 2f, tileMap.getY() + tileMap.getHeight() / 2f);
            roomObjectService.add(ladder);
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
        final Renderable roomObjects = (DrawableBatch) roomObjectService::toRender;
        return List.of(tiles, enemies, gateWalls, roomObjects);
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

    @Override
    public RoomType getRoomType() {
        return RoomType.BOSS;
    }
}
