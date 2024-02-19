package cz.empatix.roomobjects.resolvers;

import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.tiles.domain.Tile;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.Set;
import java.util.function.Predicate;

public class PositionPredicateFactory {
    private final static PhysicsService PHYSICS_SERVICE = PhysicsService.getInstance();
    public static Predicate<Vector2f> withoutAllCollisions(final Vector2i dimensions){
        return withoutAllCollisions(dimensions, new Vector2i(0, 0));
    }
    public static Predicate<Vector2f> withoutAllCollisions(final Vector2i dimensions,
                                                           final Vector2i offset){
        return position -> {
            final Vector2f positionWithOffset = new Vector2f(position).add(offset.x, offset.y);
            return PHYSICS_SERVICE.query(positionWithOffset, dimensions, Set.of()).isEmpty();
        };
    }
    public static Predicate<Vector2f> withTileCollision(final Vector2i tilesAway){
        return position -> {
            PHYSICS_SERVICE.step();

            final Vector2f positionWithOffset = new Vector2f(position)
                    .add(tilesAway.x() * Tile.SCALE_TILE_SIZE, tilesAway.y() * Tile.SCALE_TILE_SIZE);
            final Vector2i tileDimensions = new Vector2i(Tile.SCALE_TILE_SIZE, Tile.SCALE_TILE_SIZE);
                return PHYSICS_SERVICE.query(positionWithOffset, tileDimensions, Set.of())
                    .stream()
                    .anyMatch(c -> c.getUserData() instanceof Tile);
        };
    }
    public static Predicate<Vector2f> withoutTileCollision(final Vector2i tilesAway){
        return position -> {
            final Vector2f positionWithOffset = new Vector2f(position)
                    .add(tilesAway.x() * Tile.SCALE_TILE_SIZE, tilesAway.y() * Tile.SCALE_TILE_SIZE);
            final Vector2i tileDimensions = new Vector2i(Tile.SCALE_TILE_SIZE, Tile.SCALE_TILE_SIZE);
            return PHYSICS_SERVICE.query(positionWithOffset, tileDimensions, Set.of()).isEmpty();
        };
    }
}
