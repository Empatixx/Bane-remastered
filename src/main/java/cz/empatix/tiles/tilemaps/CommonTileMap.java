package cz.empatix.tiles.tilemaps;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.factory.TileFactory;
import cz.empatix.tiles.service.BooleanMapUtil;

import java.util.Arrays;
import java.util.List;
import java.util.Set;

public class CommonTileMap implements TileMap {
    private final Tile[][] tiles;
    private final int rows;
    private final int cols;
    private final int xOffset;
    private final int yOffset;

    public CommonTileMap(final int rows, final int cols,
                         final int xOffset, final int yOffset,
                         final Set<RoomDirection> directions) {
        this.rows = rows;
        this.cols = cols;
        this.xOffset = xOffset;
        this.yOffset = yOffset;

        tiles = createTiles(rows, cols, xOffset, yOffset, directions);
    }

    private Tile[][] createTiles(final int rows, final int cols, final int xOffset, final int yOffset,
                                 final Set<RoomDirection> directions) {
        final TileFactory tileFactory = new TileFactory();
        final boolean[][] map = BooleanMapUtil.createEmptyMap(rows, cols);
        BooleanMapUtil.setPaths(map, directions);
        return tileFactory.createTilemapFromClamp(BooleanMapUtil.edgeClamp(map), xOffset, yOffset);
    }

    @Override
    public List<Drawable> toRender() {
        final List<Tile> tiles = Arrays.stream(this.tiles)
                .flatMap(Arrays::stream)
                .toList();
        return List.copyOf(tiles);
    }

    @Override
    public int getWidth() {
        return cols * Tile.TILE_SIZE * Tile.SCALE;
    }

    @Override
    public int getHeight() {
        return rows * Tile.TILE_SIZE * Tile.SCALE;
    }

    @Override
    public int getX() {
        return xOffset;
    }

    @Override
    public int getY() {
        return yOffset;
    }
}
