package cz.empatix.roomobjects.suppliers;

import cz.empatix.roomobjects.domain.Banner;
import cz.empatix.roomobjects.domain.Flamethrower;
import cz.empatix.roomobjects.domain.RoomObject;
import cz.empatix.roomobjects.resolvers.*;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Vector;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Predicate;
import java.util.function.Supplier;

public class FlamethrowerSupplier implements RoomObjectSupplier<Flamethrower> {
    private final static Logger LOGGER = LoggerFactory.getLogger(FlamethrowerSupplier.class);
    private final static int MAX_TRIES = 100;
    private final TileMap tileMap;
    private final Room room;

    public FlamethrowerSupplier(final Room room,
                                final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Flamethrower supply() {
        try {
            final Flamethrower flamethrower = new Flamethrower();

            final PositionFinder positionFinder = new PositionFinder();
            final Supplier<Vector2f> positionSupplier = PositionSupplierFactory.tileBasedPositionSupplier(
                    room, tileMap, new Vector2i(2,2));
            final Predicate<Vector2f> tileCollisionPredicate = PositionPredicateFactory.withTileCollision(new Vector2i(0,0), tileMap);
            final Predicate<Vector2f> bottomNonCollisionPredicate = PositionPredicateFactory.withoutTileCollision(new Vector2i(0,1), tileMap);

            final PositionProcessor positionProcessor = new PositionProcessorImpl(List.of(tileCollisionPredicate, bottomNonCollisionPredicate));
            final Vector2f position = positionFinder.getPosition(positionSupplier, List.of(positionProcessor), MAX_TRIES);

            flamethrower.setPosition(position.x(), position.y());
            return flamethrower;
        } catch (Exception e){
            LOGGER.warn("Could not supply a flamethrower in room {}, exception: {}", room, e.getMessage());
            return null;
        }
    }
}
