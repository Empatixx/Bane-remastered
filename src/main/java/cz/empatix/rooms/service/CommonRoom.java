package cz.empatix.rooms.service;

import cz.empatix.enemies.service.EnemyService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.domain.Spike;
import cz.empatix.roomobjects.service.RoomLockService;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

public class CommonRoom implements Room {
    private static final int MAX_ENEMIES = 5;
    private static final int MAX_SPIKES = 3;
    private final EnemyService enemyService;
    private final RoomObjectService roomObjectService;
    private final ItemService itemService;
    private final RoomLockService roomLockService;
    private final Map<RoomDirection, Room> neighbors;
    private final TileMap tileMap;
    private boolean visited;

    public CommonRoom(final ItemService itemService, final int rows, final int cols, final int xOffset, final int yOffset,
                      final Set<RoomDirection> neighbors) {

        this.itemService = itemService;
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, neighbors);
        enemyService = new EnemyService(itemService);
        roomObjectService = new RoomObjectService();
        roomLockService = new RoomLockService(this, neighbors);
        visited = false;
        this.neighbors = new HashMap<>(neighbors.size());
    }

    @Override
    public void enterEvent(final Player p) {
        if (!visited) {
            visited = true;
            for (int i = 0; i < MAX_ENEMIES; i++){
                enemyService.spawnEnemy(this);
            }
            enemyService.setTarget(p);
            roomLockService.setState(true);
        }
    }

    @Override
    public void leaveEvent(final Player p) {
    }

    @Override
    public void init() {
        final int totalSpikes = ThreadLocalRandom.current().nextInt(MAX_SPIKES);
        for (int i = 0; i < totalSpikes; i++){
            final int xTile = ThreadLocalRandom.current().nextInt(2,tileMap.getHeight() / (Tile.TILE_SIZE * Tile.SCALE) - 1);
            final int yTile = ThreadLocalRandom.current().nextInt(2,tileMap.getHeight() / (Tile.TILE_SIZE * Tile.SCALE) - 1);

            final int xPos = getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
            final int yPos = getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

            final Spike spike = new Spike();
            spike.setPosition(xPos, yPos);
            roomObjectService.add(spike);
        }
    }

    @Override
    public void cleanUp() {
        roomObjectService.cleanUp();
        roomLockService.cleanUp();
        tileMap.cleanUp();
    }

    @Override
    public void update(double dt) {
        enemyService.update(dt);
        roomObjectService.update(dt);
        roomLockService.update(dt);
        if(enemyService.areEnemiesDead() && roomLockService.isLocked()){
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
        final Renderable roomObjects = (DrawableBatch) roomObjectService::toRender;
        final Renderable roomLocker = (DrawableBatch) this.roomLockService::toRender;

        final List<Renderable> renderables = new ArrayList<>(enemyService.toRender());
        renderables.add(tiles);
        renderables.add(roomObjects);
        renderables.add(roomLocker);
        return renderables;
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
        return RoomType.NORMAL;
    }

}
