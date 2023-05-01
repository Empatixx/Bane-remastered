package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;

import java.util.Arrays;
import java.util.List;

public class PathTileMap implements TileMap{
    private Tile[][] tiles;

    public final static int ROWS = 4;
    public final static int COLS = 4;

    @Override
    public List<Drawable> toRender() {
        final List<Tile> tiles = Arrays.stream(this.tiles)
                .flatMap(Arrays::stream)
                .toList();
        return List.copyOf(tiles);
    }

    @Override
    public void init() {

    }

    @Override
    public int getWidth(){
        return COLS * Tile.TILE_SIZE * Tile.SCALE;
    }
    @Override
    public int getHeight(){
        return ROWS * Tile.TILE_SIZE * Tile.SCALE;
    }
}
