package cz.empatix.roomobjects.suppliers;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsEngine;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.roomobjects.domain.Bones;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.dyn4j.geometry.Rectangle;
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

        final Bones bones = new Bones();
        final PhysicsService physicsService = PhysicsService.getInstance();

        final PhysicsObject testObject = new PhysicsObject();
        testObject.addFixture(new Rectangle(bones.getWidth(), bones.getHeight()));

        do{
            final int xTile = ThreadLocalRandom.current().nextInt(2, cols - 1);
            final int yTile = ThreadLocalRandom.current().nextInt(2, rows - 1);

            final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
            final int yPos = room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

            testObject.setTranslation(xPos, yPos);
            bones.setPosition(xPos, yPos);

        } while (physicsService.isColliding(testObject));

        return bones;
    }
}
