package cz.empatix.roomobjects.resolvers;

import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Supplier;

public class PositionSupplierFactory {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    public static Supplier<Vector2f> positionSupplier(final Room room,
                                                      final Vector2i offset){
        final int xMin = room.getMinX() + offset.x();
        final int yMin = room.getMinY() + offset.y();
        final int xMax = room.getMaxX() - offset.x();
        final int yMax = room.getMaxY() - offset.y();
        return () -> new Vector2f(RANDOM.nextInt(xMin, xMax), RANDOM.nextInt(yMin, yMax));
    }
    public static Supplier<Vector2f> tileBasedPositionSupplier(final Room room,
                                                               final TileMap tileMap,
                                                               final Vector2i offset) {
        final int minCols = offset.x();
        final int minRows = offset.y();
        final int maxCols = tileMap.getTotalCollumns() - offset.x();
        final int maxRows = tileMap.getTotalRows() - offset.y();

        final int xMin = room.getMinX();
        final int yMin = room.getMinY();
        final int tileSize = Tile.SCALE_TILE_SIZE;
        return () -> {
            final int xTile = RANDOM.nextInt(minCols, maxCols);
            final int yTile = RANDOM.nextInt(minRows, maxRows);
            return new Vector2f(xMin + xTile * tileSize, yMin + yTile * tileSize);
        };
    }
}
