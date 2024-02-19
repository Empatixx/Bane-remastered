package cz.empatix.roomobjects.suppliers;

import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.Banner;
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

public class BannerSupplier implements RoomObjectSupplier<Banner>{
    private final TileMap tileMap;
    private final Room room;
    public BannerSupplier(final Room room,
                          final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }
    @Override
    public Banner supply() {
        final Banner banner = new Banner();

        final PositionFinder positionFinder = new PositionFinder();
        final Supplier<Vector2f> positionSupplier = PositionSupplierFactory.tileBasedPositionSupplier(room, tileMap, new Vector2i(0,0));
        final Predicate<Vector2f> tileCollisionPredicate = PositionPredicateFactory.withTileCollision(new Vector2i(0,0), tileMap);
        final Predicate<Vector2f> bottomTileNonCollisionPredicate = PositionPredicateFactory.withoutTileCollision(new Vector2i(0,1), tileMap);
        final PositionProcessor processor = new PositionProcessorImpl(List.of(tileCollisionPredicate, bottomTileNonCollisionPredicate));

        final Vector2f position = positionFinder.getPosition(positionSupplier, List.of(processor));
        banner.setPosition(position.x(), position.y());
        return banner;
    }
}
