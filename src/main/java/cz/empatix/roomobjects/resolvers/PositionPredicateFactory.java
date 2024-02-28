package cz.empatix.roomobjects.resolvers;

import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.Set;
import java.util.Vector;
import java.util.function.Predicate;

public class PositionPredicateFactory {
    private final static PhysicsService PHYSICS_SERVICE = PhysicsService.getInstance();

    public static Predicate<Vector2f> withoutAllCollisions(final Vector2i dimensions) {
        return withoutAllCollisions(dimensions, new Vector2i(0, 0));
    }


    public static Predicate<Vector2f> withoutAllCollisions(final Vector2i dimensions,
                                                           final Vector2i offset) {
        return position -> {
            PHYSICS_SERVICE.updateBroadphase();
            final Vector2f positionWithOffset = new Vector2f(position).add(offset.x, offset.y);
            return PHYSICS_SERVICE.query(positionWithOffset, dimensions, Set.of()).isEmpty();
        };
    }
    private final static Vector2i TOLERANCE = new Vector2i(1, 1);
    public static Predicate<Vector2f> withTileCollision(final Vector2i tilesAway, final TileMap tileMap) {
        return position -> {
            final Vector2f positionWithOffset = new Vector2f(position)
                    .add(tilesAway.x() * Tile.SCALE_TILE_SIZE, tilesAway.y() * Tile.SCALE_TILE_SIZE)
                    .add(TOLERANCE.x(), TOLERANCE.y());
            if (!isInsideTileMap(positionWithOffset, tileMap)) {
                return true;
            }
            final Vector2i tileDimensions = new Vector2i(Tile.SCALE_TILE_SIZE, Tile.SCALE_TILE_SIZE)
                    .sub(new Vector2i(TOLERANCE.x() * 2, TOLERANCE.y() * 2));
            PHYSICS_SERVICE.updateBroadphase();
            return PHYSICS_SERVICE.query(positionWithOffset, tileDimensions, Set.of())
                    .stream()
                    .anyMatch(c -> c.getUserData() instanceof Tile);
        };
    }

    public static Predicate<Vector2f> withoutTileCollision(final Vector2i tilesAway, final TileMap tileMap) {
        return position -> {
            final Vector2f positionWithOffset = new Vector2f(position)
                    .add(tilesAway.x() * Tile.SCALE_TILE_SIZE, tilesAway.y() * Tile.SCALE_TILE_SIZE)
                    .add(TOLERANCE.x(), TOLERANCE.y());
            if (!isInsideTileMap(positionWithOffset, tileMap)) {
                return false;
            }
            final Vector2i tileDimensions = new Vector2i(Tile.SCALE_TILE_SIZE, Tile.SCALE_TILE_SIZE)
                    .sub(new Vector2i(TOLERANCE.x() * 2, TOLERANCE.y() * 2));
            PHYSICS_SERVICE.updateBroadphase();
            return PHYSICS_SERVICE.query(positionWithOffset, tileDimensions, Set.of()).isEmpty();
        };
    }

    private static boolean isInsideTileMap(final Vector2f pos, final TileMap tileMap) {
        final float xMin = tileMap.getX();
        final float xMax = tileMap.getX() + tileMap.getWidth();
        final float yMin = tileMap.getY();
        final float yMax = tileMap.getY() + tileMap.getHeight();
        return pos.x() > xMin && pos.x() < xMax && pos.y() > yMin && pos.y() < yMax;
    }
}
