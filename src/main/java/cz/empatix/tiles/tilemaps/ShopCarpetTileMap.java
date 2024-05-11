package cz.empatix.tiles.tilemaps;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.factory.CarpetTileFactory;

import java.util.Arrays;
import java.util.List;
import java.util.Set;

public class ShopCarpetTileMap implements TileMap{
    private final Tile[][] tiles;
    private final int rows;
    private final int cols;
    private final int xOffset;
    private final int yOffset;

    public ShopCarpetTileMap(final int rows, final int cols,
                                 final int xOffset, final int yOffset,
                                 final Set<RoomDirection> directions) {
        this.rows = rows;
        this.cols = cols;
        this.xOffset = xOffset;
        this.yOffset = yOffset;

        final CarpetTileFactory tileFactory = new CarpetTileFactory();

        final boolean[][] map = new boolean[rows][cols];
        for (int i = 4; i < rows - 4; i++) {
            for (int j = 2; j < cols - 2; j++) {
                map[i][j] = true;
            }
        }
        final Tile[][] carpetTiles = tileFactory.createTileMap(map, xOffset, yOffset);
        tiles = carpetTiles;
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
        return cols * TileImpl.TILE_SIZE * TileImpl.SCALE;
    }

    @Override
    public int getHeight() {
        return rows * TileImpl.TILE_SIZE * TileImpl.SCALE;
    }

    @Override
    public int getX() {
        return xOffset;
    }

    @Override
    public int getY() {
        return yOffset;
    }

    @Override
    public void cleanUp() {
        Arrays.stream(tiles)
                .flatMap(Arrays::stream)
                .forEach(Tile::destroy);

    }
}
