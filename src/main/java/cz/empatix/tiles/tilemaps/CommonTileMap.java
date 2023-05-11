package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.factory.TileFactory;
import cz.empatix.tiles.domain.Tile;

import java.util.Arrays;
import java.util.List;

public class CommonTileMap implements TileMap {
    private Tile[][] tiles;
    private final int rows;
    private final int cols;
    public CommonTileMap(final int rows, final int cols){
        this.rows = rows;
        this.cols = cols;
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
    /*map[5][5] = true;
        map[6][5] = true;
        map[5][6] = true;
        map[6][6] = true;
*/
        tiles = tileFactory.createTileMap(map);

    }
    @Override
    public int getWidth(){
        return cols * Tile.TILE_SIZE * Tile.SCALE;
    }
    @Override
    public int getHeight(){
        return rows * Tile.TILE_SIZE * Tile.SCALE;
    }
}
