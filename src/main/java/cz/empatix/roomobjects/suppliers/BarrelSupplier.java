package cz.empatix.roomobjects.suppliers;

import cz.empatix.items.service.ItemService;
import cz.empatix.roomobjects.domain.Barrel;
import cz.empatix.roomobjects.resolvers.*;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;
import java.util.function.Predicate;
import java.util.function.Supplier;

public class BarrelSupplier implements RoomObjectSupplier<Barrel> {
    private final Room room;
    private final TileMap tileMap;
    private final ItemService itemService;

    public BarrelSupplier(final Room room,
                          final TileMap tileMap,
                          final ItemService itemService) {
        this.room = room;
        this.tileMap = tileMap;
        this.itemService = itemService;
    }

    @Override
    public Barrel supply() {
        final Barrel barrel = new Barrel(itemService);
        final PositionFinder positionFinder = new PositionFinder();

        final Supplier<Vector2f> positionSupplier = PositionSupplierFactory.positionSupplier(room, new Vector2i(64, 64));
        final Predicate<Vector2f> collisionPredicate = PositionPredicateFactory.withoutAllCollisions(new Vector2i(barrel.getWidth(), barrel.getHeight()));
        final Predicate<Vector2f> withoutRoomObjectCollisions = PositionPredicateFactory.withoutRoomObjectCollisions(barrel, room);
        final PositionProcessor positionResolver = new PositionProcessorImpl(List.of(collisionPredicate));

        final Vector2f position = positionFinder.getPosition(positionSupplier, List.of(positionResolver));
        barrel.setPosition(position.x(), position.y());
        return barrel;
    }
}
