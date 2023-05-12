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
    private Tile[][] tiles;

    public PathTileMap(final int xOffset,
                       final int yOffset,
                       final int rows,
                       final int cols) {
        this.xOffset = xOffset;
        this.yOffset = yOffset;
        this.cols = cols;
        this.rows = rows;
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
        tiles = new Tile[0][0];
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
