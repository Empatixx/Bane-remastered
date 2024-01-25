package cz.empatix.roomobjects.suppliers;

import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.roomobjects.domain.Spike;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;

import java.util.concurrent.ThreadLocalRandom;

public class SpikeSupplier implements RoomObjectSupplier<Spike> {
    private final Room room;
    private final TileMap tileMap;

    public SpikeSupplier(final Room room, final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Spike supply() {
        final int cols = tileMap.getTotalCollumns();
        final int rows = tileMap.getTotalRows();

        final int xTile = ThreadLocalRandom.current().nextInt(2, cols - 1);
        final int yTile = ThreadLocalRandom.current().nextInt(2, rows - 1);

        final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
        final int yPos = room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

        final Spike spike = new Spike();
        spike.setPosition(xPos, yPos);

        final Rectangle spikeRectangle = new Rectangle(spike.getWidth(), spike.getHeight());

        final PhysicsService physicsService = PhysicsService.getInstance();
        while (physicsService.isColliding(spikeRectangle,new Vector2f(xPos, yPos))) {
            final int xTileNew = ThreadLocalRandom.current().nextInt(2, tileMap.getWidth() / (Tile.TILE_SIZE * Tile.SCALE) - 1);
            final int yTileNew = ThreadLocalRandom.current().nextInt(2, tileMap.getHeight() / (Tile.TILE_SIZE * Tile.SCALE) - 1);

            final int xPosNew = room.getX() + xTileNew * Tile.TILE_SIZE * Tile.SCALE;
            final int yPosNew = room.getY() + yTileNew * Tile.TILE_SIZE * Tile.SCALE;

            spike.setPosition(xPosNew, yPosNew);
        }
        return spike;
    }
}
