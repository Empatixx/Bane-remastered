package cz.empatix.tiles.tilemaps;

import cz.empatix.common.RngService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.factory.TileFactory;
import cz.empatix.tiles.service.BooleanMapUtil;

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

        createTiles(xOffset, yOffset, rows, cols, horizontal);
    }

    private void createTiles(int xOffset, int yOffset, int rows, int cols, boolean horizontal) {
        final boolean[][] map = new boolean[rows][cols];
        if(horizontal){
            for(int i = 0; i < cols; i++){
                map[0][i] = true;
                map[rows - 1][i] = true;
            }
        } else {
            for(int i = 0; i < rows; i++){
                map[i][0] = true;
                map[i][cols - 1] = true;
            }
        }
        final TileFactory tileFactory = new TileFactory(new RngService(RngService.seedForCoordinate(xOffset, yOffset)));
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
    public int getWidth(){
        return cols * TileImpl.TILE_SIZE * TileImpl.SCALE;
    }
    @Override
    public int getHeight(){
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
