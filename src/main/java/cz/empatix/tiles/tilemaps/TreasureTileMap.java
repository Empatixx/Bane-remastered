package cz.empatix.tiles.tilemaps;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.service.BooleanMapUtil;

import java.util.Arrays;
import java.util.List;

public class TreasureTileMap implements TileMap {
    private Tile[][] tiles;
    private final int rows;
    private final int cols;
    private final int xOffset;
    private final int yOffset;

    public TreasureTileMap(final int rows, final int cols,
                           final int xOffset, final int yOffset,
                           final List<RoomDirection> directions) {
        this.rows = rows;
        this.cols = cols;
        this.xOffset = xOffset;
        this.yOffset = yOffset;

        final boolean[][] map = BooleanMapUtil.createEmptyMap(rows, cols);
        BooleanMapUtil.setPaths(map, directions);

        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if ((i == 3 || i == 4 || i == rows - 4 || i == rows - 5) && (j == 3 || j == 4 || j == cols - 4 || j == cols - 5)) {
                    map[i][j] = true;
                }
            }
        }

        tiles = tileFactory.createTilemapFromClamp(BooleanMapUtil.edgeClamp(map), xOffset, yOffset);
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
