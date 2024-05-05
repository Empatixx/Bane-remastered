package cz.empatix.roomobjects.suppliers;

import cz.empatix.roomobjects.domain.Spike;
import cz.empatix.roomobjects.resolvers.*;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;
import java.util.function.Predicate;
import java.util.function.Supplier;

public class SpikeSupplier implements RoomObjectSupplier<Spike> {
    private final Room room;
    private final TileMap tileMap;

    public SpikeSupplier(final Room room, final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Spike supply() {
        final Spike spike = new Spike();

        final PositionFinder positionFinder = new PositionFinder();
        final Supplier<Vector2f> positionSupplier = PositionSupplierFactory.tileBasedPositionSupplier(
                room, tileMap, new Vector2i(2,2));
        final Predicate<Vector2f> collisionPredicate = PositionPredicateFactory.withoutAllCollisions(
                new Vector2i(spike.getWidth(), spike.getHeight()));
        final Predicate<Vector2f> withoutRoomObjectCollisions = PositionPredicateFactory.withoutRoomObjectCollisions(spike, room);
        final PositionProcessor processor = new PositionProcessorImpl(List.of(collisionPredicate, withoutRoomObjectCollisions));

        final Vector2f position = positionFinder.getPosition(positionSupplier, List.of(processor));
        spike.setPosition(position.x(), position.y());
        return spike;
    }
}
