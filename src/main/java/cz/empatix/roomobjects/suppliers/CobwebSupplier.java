package cz.empatix.roomobjects.suppliers;

import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.Cobweb;
import cz.empatix.roomobjects.resolvers.*;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Function;
import java.util.function.Predicate;
import java.util.function.Supplier;

public class CobwebSupplier implements RoomObjectSupplier<Cobweb>{
    private final Room room;
    private final TileMap tileMap;

    public CobwebSupplier(final Room room, final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Cobweb supply() {
        final Cobweb cobweb = new Cobweb();

        final PositionFinder positionFinder = new PositionFinder();
        final Supplier<Vector2f> positionSupplier = PositionSupplierFactory.tileBasedPositionSupplier(room, tileMap, new Vector2i(0,0));

        final Predicate<Vector2f> tileCollisionPredicate = PositionPredicateFactory.withTileCollision(new Vector2i(0,0), tileMap);
        final Predicate<Vector2f> rightTileNonCollisionPredicate = PositionPredicateFactory.withoutTileCollision(new Vector2i(1,0), tileMap);
        final Predicate<Vector2f> leftTileNonCollisionPredicate = PositionPredicateFactory.withoutTileCollision(new Vector2i(-1,0), tileMap);

        final PositionProcessor rightProcessor = new PositionProcessorImpl(List.of(tileCollisionPredicate, rightTileNonCollisionPredicate), Function.identity(), pos -> pos.add(Tile.SCALE_TILE_SIZE, 0));
        final PositionProcessor leftProcessor = new PositionProcessorImpl(List.of(tileCollisionPredicate, leftTileNonCollisionPredicate), Function.identity(), pos -> {
            cobweb.setLeft();
            return pos.add(-Tile.SCALE_TILE_SIZE, 0);
        });

        final Vector2f position = positionFinder.getPosition(positionSupplier, List.of(rightProcessor, leftProcessor));
        cobweb.setPosition(position.x(), position.y());
        return cobweb;
    }
}
