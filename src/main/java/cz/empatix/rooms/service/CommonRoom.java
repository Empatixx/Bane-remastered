package cz.empatix.rooms.service;

import cz.empatix.enemies.service.EnemyService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.physics.domain.geometry.Rectangle;
import cz.empatix.physics.service.PhysicsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.Spike;
import cz.empatix.roomobjects.domain.Torch;
import cz.empatix.roomobjects.service.RoomLockService;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.ProceduralTileMap;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

public class CommonRoom implements Room {
    private static final int MAX_ENEMIES = 5;
    private static final int MAX_SPIKES = 3;
    private static final int MAX_TORCHES = 2;
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
        tileMap = new ProceduralTileMap(rows, cols, xOffset, yOffset, neighbors);
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
            for (int i = 0; i < MAX_ENEMIES; i++) {
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
        for (int i = 0; i < totalSpikes; i++) {
            final int cols = tileMap.getTotalCollumns();
            final int rows = tileMap.getTotalRows();

            final int xTile = ThreadLocalRandom.current().nextInt(2, cols - 1);
            final int yTile = ThreadLocalRandom.current().nextInt(2, rows - 1);

            final int xPos = getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
            final int yPos = getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

            final Spike spike = new Spike();
            spike.setPosition(xPos, yPos);

            final Vector2f spikePosition = new Vector2f(xPos, yPos);
            final Rectangle spikeRectangle = new Rectangle(spike.getWidth(), spike.getHeight());

            final PhysicsService physicsService = PhysicsService.getInstance();
            while (physicsService.isColliding(spikePosition, spikeRectangle)) {
                final int xTileNew = ThreadLocalRandom.current().nextInt(2, tileMap.getWidth() / (Tile.TILE_SIZE * Tile.SCALE) - 1);
                final int yTileNew = ThreadLocalRandom.current().nextInt(2, tileMap.getHeight() / (Tile.TILE_SIZE * Tile.SCALE) - 1);

                final int xPosNew = getX() + xTileNew * Tile.TILE_SIZE * Tile.SCALE;
                final int yPosNew = getY() + yTileNew * Tile.TILE_SIZE * Tile.SCALE;

                spikePosition.set(xPosNew, yPosNew);
            }
            spike.setPosition(spikePosition.x(), spikePosition.y());
            roomObjectService.add(spike);
        }
        for (int i = 0; i < MAX_TORCHES; i++) {
            boolean found = false;
            while (!found) {
                final int cols = tileMap.getTotalCollumns();
                final int rows = tileMap.getTotalRows();

                final int xTile = ThreadLocalRandom.current().nextInt(0, cols - 1);
                final int yTile = ThreadLocalRandom.current().nextInt(0, rows - 1);

                if (!tileMap.isWall(xTile, yTile)) {
                    if (xTile + 1 < cols && tileMap.isWall(xTile + 1, yTile)) {
                        found = true;
                        final int xPos = getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                        final int yPos = getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

                        final Torch torch = new Torch();
                        torch.setPosition(xPos, yPos);
                        torch.setState(AnimationState.RIGHT_SIDE);
                        roomObjectService.add(torch);

                    }
                    else if (xTile - 1 >= 0 && tileMap.isWall(xTile - 1, yTile)) {
                        found = true;
                        final int xPos = getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                        final int yPos = getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

                        final Torch torch = new Torch();
                        torch.setPosition(xPos, yPos);
                        torch.setState(AnimationState.LEFT_SIDE);
                        roomObjectService.add(torch);

                    }
                    else if (yTile - 1 >= 0 && tileMap.isWall(xTile, yTile - 1)) {
                        found = true;
                        final int xPos = getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                        final int yPos = getY() + (yTile - 1) * Tile.TILE_SIZE * Tile.SCALE;

                        final Torch torch = new Torch();
                        torch.setPosition(xPos, yPos);
                        torch.setState(AnimationState.TOP);
                        roomObjectService.add(torch);
                    }
                }

            }
        }
    }

    @Override
    public void cleanUp() {
        enemyService.cleanUp();
        roomObjectService.cleanUp();
        roomLockService.cleanUp();
        tileMap.cleanUp();
    }

    @Override
    public void update(double dt) {
        enemyService.update(dt);
        roomObjectService.update(dt);
        roomLockService.update(dt);
        if (enemyService.areEnemiesDead() && roomLockService.isLocked()) {
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
