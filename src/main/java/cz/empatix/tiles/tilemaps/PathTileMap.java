package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;

import java.util.Arrays;
import java.util.List;

public class PathTileMap implements TileMap{
    private final int cols;
    private final int rows;
    private final int xOffset;
    private final int yOffset;
    private final boolean horizontal;
    private Tile[][] tiles;

    public PathTileMap(final int xOffset,
                       final int yOffset,
                       final int rows,
                       final int cols,
                       final boolean horizontal) {
        this.xOffset = xOffset;
        this.yOffset = yOffset;
        this.cols = cols;
        this.rows = rows;
        this.horizontal = horizontal;
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
        final boolean[][] map = new boolean[rows][cols];
        if(horizontal){
            for(int i = 0; i < rows; i++){
                map[i][0] = true;
            }
        } else {
            for(int i = 0; i < cols; i++){
                map[0][i] = true;
            }
        }
        tiles = tileFactory.createTileMap(map, xOffset, yOffset);
    }

    @Override
    public int getWidth(){
        return cols * Tile.TILE_SIZE * Tile.SCALE;
    }
    @Override
    public int getHeight(){
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
