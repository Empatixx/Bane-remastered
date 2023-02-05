package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.TileFactory;
import cz.empatix.tiles.domain.Tile;

import java.util.Arrays;
import java.util.List;

public class CommonTileMap implements TileMap {
    private Tile[][] tiles;
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
        boolean[][] map = new boolean[20][20];
        for(int i = 0;i<20;i++){
            for(int j = 0;j<20;j++){
                if(i == 0 || j == 0 || i == 19 || j == 19) {
                    map[i][j] = true;
                }
            }
        }
        map[5][5] = true;
        map[6][5] = true;
        map[5][6] = true;
        map[6][6] = true;

        tiles = tileFactory.createTileMap(map);

    }
}
