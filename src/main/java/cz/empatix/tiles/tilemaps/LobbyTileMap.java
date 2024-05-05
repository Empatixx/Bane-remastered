package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.factory.TileFactory;
import cz.empatix.tiles.service.BooleanMapUtil;

import java.util.Arrays;
import java.util.List;

public class LobbyTileMap implements TileMap {
    private final static int COLS = 29;
    private final static int ROWS = 10;
    private final Tile[][] tiles;
    public LobbyTileMap() {
        tiles = createTiles();
    }

    private Tile[][] createTiles() {
        final TileFactory tileFactory = new TileFactory();
        final boolean[][] map = BooleanMapUtil.createEmptyMap(ROWS, COLS);
        for(int i = 11; i < 13; i++){
            map[1][i] = true;
            map[2][i] = true;
            map[ROWS - 3][i] = true;
            map[ROWS - 2][i] = true;
        }
        for(int i = 16; i < 18; i++){
            map[1][i] = true;
            map[2][i] = true;
            map[ROWS - 3][i] = true;
            map[ROWS - 2][i] = true;
        }
        for(int i = 11;i < 18; i++){
            map[ROWS - 2][i] = true;
        }
        return tileFactory.createTileMap(map, 0, 0);
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
        return COLS * TileImpl.TILE_SIZE * TileImpl.SCALE;
    }

    @Override
    public int getHeight() {
        return ROWS * TileImpl.TILE_SIZE * TileImpl.SCALE;
    }

    @Override
    public int getX() {
        return 0;
    }

    @Override
    public int getY() {
        return 0;
    }

    @Override
    public void cleanUp() {
        Arrays.stream(tiles)
                .flatMap(Arrays::stream)
                .forEach(Tile::destroy);
    }
}
