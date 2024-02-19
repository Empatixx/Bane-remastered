package cz.empatix.roomobjects.suppliers;

import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.Torch;
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

public class TorchSupplier implements RoomObjectSupplier<Torch> {
    private final Room room;
    private final TileMap tileMap;

    public TorchSupplier(final Room room, final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Torch supply() {
        final Torch torch = new Torch();

        final PositionFinder positionFinder = new PositionFinder();
        final Supplier<Vector2f> positionSupplier = PositionSupplierFactory.tileBasedPositionSupplier(
                room, tileMap, new Vector2i(0,0));
        final Predicate<Vector2f> tileCollisionPredicate = PositionPredicateFactory.withTileCollision(new Vector2i(0, 0));
        final Predicate<Vector2f> bottomTileNonCollisionPredicate = PositionPredicateFactory.withoutTileCollision(new Vector2i(0, 1));
        final Predicate<Vector2f> leftTileNonCollisionPredicate = PositionPredicateFactory.withoutTileCollision(new Vector2i(-1, 0));
        final Predicate<Vector2f> rightTileNonCollisionPredicate = PositionPredicateFactory.withoutTileCollision(new Vector2i(1, 0));

        final PositionProcessor bottomProcessor = new PositionProcessorImpl(List.of(tileCollisionPredicate, bottomTileNonCollisionPredicate),
                Function.identity(), pos -> {
            torch.setState(AnimationState.TOP);
            return pos.add(0, -Tile.TILE_SIZE * Tile.SCALE);
        });
        final PositionProcessor leftProcessor = new PositionProcessorImpl(List.of(tileCollisionPredicate, leftTileNonCollisionPredicate),
                Function.identity(), pos -> {
            torch.setState(AnimationState.RIGHT_SIDE);
            return pos;
        });
        final PositionProcessor rightProcessor = new PositionProcessorImpl(List.of(tileCollisionPredicate, rightTileNonCollisionPredicate),
                Function.identity(), pos -> {
            torch.setState(AnimationState.LEFT_SIDE);
            return pos;
        });
        final Vector2f position = positionFinder.getPosition(positionSupplier, List.of(bottomProcessor, leftProcessor, rightProcessor));
        torch.setPosition(position.x(), position.y());
        return torch;
    }
}
