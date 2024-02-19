package cz.empatix.roomobjects.suppliers;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsEngine;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.roomobjects.domain.Bones;
import cz.empatix.roomobjects.resolvers.*;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Predicate;
import java.util.function.Supplier;

public class  BonesSupplier implements RoomObjectSupplier<Bones>{
    private final Room room;
    private final TileMap tileMap;

    public BonesSupplier(final Room room, final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Bones supply() {
        final Bones bones = new Bones();

        final PositionFinder positionFinder = new PositionFinder();
        final Supplier<Vector2f> positionSupplier = PositionSupplierFactory.tileBasedPositionSupplier(room, tileMap, new Vector2i(1,1));
        final Predicate<Vector2f> collisionPredicate = PositionPredicateFactory.withoutAllCollisions(new Vector2i(bones.getWidth(), bones.getHeight()));
        final PositionProcessor positionResolver = new PositionProcessorImpl(List.of(collisionPredicate));

        final Vector2f position = positionFinder.getPosition(positionSupplier, List.of(positionResolver));
        bones.setPosition(position.x(), position.y());
        return bones;
    }
}
