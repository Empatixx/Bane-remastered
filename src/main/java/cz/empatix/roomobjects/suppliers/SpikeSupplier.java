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

        final Spike spike = new Spike();
        final PhysicsService physicsService = PhysicsService.getInstance();
        do{
            final int xTile = ThreadLocalRandom.current().nextInt(2, cols - 1);
            final int yTile = ThreadLocalRandom.current().nextInt(2, rows - 1);

            final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
            final int yPos = room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

            spike.setPosition(xPos, yPos);
        } while (physicsService.isColliding(spike.getPhysicsObject()));

        return spike;
    }
}
