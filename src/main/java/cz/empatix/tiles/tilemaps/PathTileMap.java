package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.factory.TileFactory;

import java.util.Arrays;
import java.util.List;

public class PathTileMap implements TileMap{
    private static final int WRAPPER = 2;
    private static final int WRAPPER_START = 0;
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
        final boolean[][] map = new boolean[rows+WRAPPER][cols+WRAPPER];

        final int wrapperSize = WRAPPER / 2;
        final int wrapperStart = 0;

        for(int i = wrapperStart; i < rows+WRAPPER; i++){
            for(int j = wrapperStart; j < cols+WRAPPER; j++){
                if(horizontal){
                    if(i == wrapperStart || i == 1 || i == rows+WRAPPER-1 || i == rows+WRAPPER-2){
                        map[i][j] = true;
                    }
                } else {
                    if(j == wrapperStart || j == 1 || j == cols+WRAPPER-1 || j == cols+WRAPPER-2){
                        map[i][j] = true;
                    }
                }
            }
        }
        final int tileSize = Tile.TILE_SIZE * Tile.SCALE;
        final Tile[][] tileMap = new TileFactory().createTileMap(map, xOffset-tileSize, yOffset-tileSize);

        this.tiles = new Tile[rows][cols];
        for(int i = wrapperSize; i < rows+wrapperSize; i++){
            for(int j = wrapperSize; j < cols+wrapperSize; j++){
                this.tiles[i-wrapperSize][j-wrapperSize] = tileMap[i][j];
            }
        }
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
