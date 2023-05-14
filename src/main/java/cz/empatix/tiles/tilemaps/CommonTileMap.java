package cz.empatix.tiles.tilemaps;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.factory.TileFactory;

import java.util.Arrays;
import java.util.List;

public class CommonTileMap implements TileMap {
    private Tile[][] tiles;
    private final int rows;
    private final int cols;
    private final int xOffset;
    private final int yOffset;
    private final List<RoomDirection> roomDirections;

    public CommonTileMap(final int rows, final int cols,
                         final int xOffset, final int yOffset) {
        this.rows = rows;
        this.cols = cols;
        this.xOffset = xOffset;
        this.yOffset = yOffset;
        this.roomDirections = List.of();
    }

    public CommonTileMap(final int rows, final int cols,
                         final int xOffset, final int yOffset,
                         final List<RoomDirection> roomDirections) {
        this.rows = rows;
        this.cols = cols;
        this.xOffset = xOffset;
        this.yOffset = yOffset;
        this.roomDirections = roomDirections;
    }

    @Override
    public List<Drawable> toRender() {
        final List<Tile> tiles = Arrays.stream(this.tiles)
                .flatMap(Arrays::stream)
                .toList();
        return List.copyOf(tiles);
    }

    @Override
    public void init() {
        final TileFactory tileFactory = new TileFactory();
        boolean[][] map = new boolean[rows][cols];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (i == 0 || j == 0 || i == rows - 1 || j == cols - 1) {
                    map[i][j] = true;
                }
            }
        }
        for (final RoomDirection dir : roomDirections) {
            switch (dir) {
                case BOTTOM -> {
                    final int centerCols = (this.cols - 4) / 2;
                    map[rows - 1][centerCols + 1] = false;
                    map[rows - 1][centerCols + 2] = false;
                }
                case TOP -> {
                    final int centerCols = (this.cols - 4) / 2;
                    map[0][centerCols + 1] = false;
                    map[0][centerCols + 2] = false;
                }
                case LEFT -> {
                    final int centerRows = (this.rows - 4) / 2;
                    map[centerRows + 1][0] = false;
                    map[centerRows + 2][0] = false;
                }
                case RIGHT -> {
                    final int centerRows = (this.rows - 4) / 2;
                    map[centerRows + 1][cols - 1] = false;
                    map[centerRows + 2][cols - 1] = false;
                }
            }
        }
    /*map[5][5] = true;
        map[6][5] = true;
        map[5][6] = true;
        map[6][6] = true;
*/
        tiles = tileFactory.createTileMap(map, xOffset, yOffset);

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
