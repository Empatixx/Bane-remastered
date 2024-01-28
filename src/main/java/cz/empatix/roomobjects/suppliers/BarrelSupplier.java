package cz.empatix.roomobjects.suppliers;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.roomobjects.domain.Barrel;
import cz.empatix.roomobjects.domain.Bones;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.dyn4j.geometry.Rectangle;

import java.util.concurrent.ThreadLocalRandom;

public class BarrelSupplier implements RoomObjectSupplier<Barrel>{
    private final static int MAX_TRIES = 100;
    private final Room room;
    private final TileMap tileMap;
    public BarrelSupplier(final Room room,
                          final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }
    @Override
    public Barrel supply() {
        final int cols = tileMap.getTotalCollumns();
        final int rows = tileMap.getTotalRows();

        final Barrel barrel = new Barrel();
        final PhysicsService physicsService = PhysicsService.getInstance();

        final PhysicsObject testObject = new PhysicsObject();
        testObject.addFixture(new Rectangle(barrel.getCollissionWidth(), barrel.getCollissionHeight()));

        int tries = 0;

        do{
            final int xTile = ThreadLocalRandom.current().nextInt(1, cols - 1);
            final int yTile = ThreadLocalRandom.current().nextInt(1, rows - 1);

            final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
            final int yPos = room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

            testObject.setTranslation(xPos, yPos);
            tries++;
        } while (physicsService.isColliding(testObject) && tries < MAX_TRIES);

        barrel.setPosition(testObject.getPhysicsTranslationX(), testObject.getPhysicsTranslationY());
        return barrel;
    }
}
