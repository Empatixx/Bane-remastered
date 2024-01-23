package cz.empatix.roomobjects.suppliers;

import cz.empatix.physics.domain.geometry.Rectangle;
import cz.empatix.physics.service.PhysicsService;
import cz.empatix.roomobjects.domain.Bones;
import cz.empatix.roomobjects.domain.Spike;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;

import java.util.concurrent.ThreadLocalRandom;

public class BonesSupplier implements RoomObjectSupplier<Bones>{
    private final Room room;
    private final TileMap tileMap;

    public BonesSupplier(final Room room, final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Bones supply() {
        final int cols = tileMap.getTotalCollumns();
        final int rows = tileMap.getTotalRows();

        final int xTile = ThreadLocalRandom.current().nextInt(2, cols - 1);
        final int yTile = ThreadLocalRandom.current().nextInt(2, rows - 1);

        final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
        final int yPos = room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

        final Bones bones = new Bones();
        bones.setPosition(xPos, yPos);

        final Vector2f spikePosition = new Vector2f(xPos, yPos);
        final Rectangle spikeRectangle = new Rectangle(bones.getWidth(), bones.getHeight());

        final PhysicsService physicsService = PhysicsService.getInstance();
        while (physicsService.isColliding(spikePosition, spikeRectangle)) {
            final int xTileNew = ThreadLocalRandom.current().nextInt(2, tileMap.getWidth() / (Tile.TILE_SIZE * Tile.SCALE) - 1);
            final int yTileNew = ThreadLocalRandom.current().nextInt(2, tileMap.getHeight() / (Tile.TILE_SIZE * Tile.SCALE) - 1);

            final int xPosNew = room.getX() + xTileNew * Tile.TILE_SIZE * Tile.SCALE;
            final int yPosNew = room.getY() + yTileNew * Tile.TILE_SIZE * Tile.SCALE;

            spikePosition.set(xPosNew, yPosNew);
        }
        bones.setPosition(spikePosition.x(), spikePosition.y());
        return bones;
    }
}
