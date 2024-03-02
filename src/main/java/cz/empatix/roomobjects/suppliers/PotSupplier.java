package cz.empatix.roomobjects.suppliers;

import cz.empatix.roomobjects.domain.Barrel;
import cz.empatix.roomobjects.domain.Pot;
import cz.empatix.roomobjects.resolvers.*;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;
import java.util.function.Predicate;
import java.util.function.Supplier;

public class PotSupplier implements RoomObjectSupplier<Pot>{
    private final Room room;
    private final TileMap tileMap;

    public PotSupplier(final Room room,
                          final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Pot supply() {
        final Pot pot = new Pot();
        final PositionFinder positionFinder = new PositionFinder();

        final Supplier<Vector2f> positionSupplier = PositionSupplierFactory.positionSupplier(room, new Vector2i(64, 64));
        final Predicate<Vector2f> collisionPredicate = PositionPredicateFactory.withoutAllCollisions(new Vector2i(pot.getWidth(), pot.getHeight()));
        final Predicate<Vector2f> withoutRoomObjectCollisions = PositionPredicateFactory.withoutRoomObjectCollisions(pot, room);
        final PositionProcessor positionResolver = new PositionProcessorImpl(List.of(collisionPredicate, withoutRoomObjectCollisions));

        final Vector2f position = positionFinder.getPosition(positionSupplier, List.of(positionResolver));
        pot.setPosition(position.x(), position.y());
        return pot;
    }
}
