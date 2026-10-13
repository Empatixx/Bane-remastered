package cz.empatix.tiles.tilemaps;

import cz.empatix.common.RngService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.factory.TileFactory;
import cz.empatix.tiles.service.BooleanMapUtil;
import cz.empatix.tiles.service.PerlinNoiseGenerator;

import java.util.Arrays;
import java.util.List;
import java.util.Set;

public class ProceduralTileMap implements TileMap {
    private final Tile[][] tiles;
    private final int rows;
    private final int cols;
    private final int xOffset;
    private final int yOffset;

    public ProceduralTileMap(final int rows, final int cols,
                             final int xOffset, final int yOffset,
                             final Set<RoomDirection> directions) {
        this.rows = rows;
        this.cols = cols;
        this.xOffset = xOffset;
        this.yOffset = yOffset;

        tiles = createTiles(rows, cols, xOffset, yOffset, directions);
    }

    private Tile[][] createTiles(final int rows, final int cols, final int xOffset, final int yOffset,
                                     final Set<RoomDirection> directions) {
        final long tileSeed = RngService.seedForCoordinate(xOffset, yOffset);
        final TileFactory tileFactory = new TileFactory(new RngService(tileSeed));
        final boolean[][] map = BooleanMapUtil.createEmptyMap(rows, cols);
        applyPerlinNoiseToMap(map, 0.05, 0.2, 0.4, tileSeed);
        removeSingleWalls(map);
        BooleanMapUtil.setPaths(map, directions);
        return tileFactory.createTilemapFromClamp(BooleanMapUtil.edgeClamp(map), xOffset, yOffset);
    }

    private void applyPerlinNoiseToMap(boolean[][] map, final double scale, final double lowerThreshold, final double upperThreshold, final long seed) {
        final PerlinNoiseGenerator perlinNoiseGenerator = new PerlinNoiseGenerator(seed);

        for (int y = 0; y < map.length; y++) {
            for (int x = 0; x < map[0].length; x++) {
                double noiseValue = perlinNoiseGenerator.noise(x * scale, y * scale);
                if (noiseValue > lowerThreshold && noiseValue < upperThreshold && isSuitableForWall(map, x, y)) {
                    map[y][x] = true; // Set to wall
                }
            }
        }
    }

    /**
     * removes walls(1) that are surrounded by empty tiles(0) on each side or by only one wall on some side
     */
    private void removeSingleWalls(final boolean[][] map) {
        boolean changed = false;
        do{
            changed = false;
            for (int y = 1; y < map.length - 1; y++) {
                for (int x = 1; x < map[0].length - 1; x++) {

                    if (map[y][x] && !isValidWall(map, x, y)) {
                        map[y][x] = false;
                        changed = true;
                    }
                }
            }
        } while (changed);
    }

    public boolean isValidWall(boolean[][] map, int x, int y) {
        final boolean top = y - 1 >= 0 && map[y - 1][x];
        final boolean bottom = y + 1 < map.length && map[y + 1][x];
        final boolean left = x - 1 >= 0 && map[y][x - 1];
        final boolean right = x + 1 < map[0].length && map[y][x + 1];

        final boolean topLeftCorner = y - 1 >= 0 && x - 1 >= 0 && map[y - 1][x - 1];
        final boolean topRightCorner = y - 1 >= 0 && x + 1 < map[0].length && map[y - 1][x + 1];
        final boolean bottomLeftCorner = y + 1 < map.length && x - 1 >= 0 && map[y + 1][x - 1];
        final boolean bottomRightCorner = y + 1 < map.length && x + 1 < map[0].length && map[y + 1][x + 1];
        if(top && (!left && !right)){
            return false;
        }
        if (bottom && (!left && !right)) {
            return false;
        }
        if (!top && !bottom) {
            return false;
        }
        if (!top && ((!left && !topLeftCorner && !bottomRightCorner) || (!right && !topRightCorner && !bottomLeftCorner))) {
            return false;
        }
        if (!bottom && ((!left && !bottomLeftCorner && !topRightCorner) || (!right && !bottomRightCorner  && !topLeftCorner))) {
            return false;
        }
        return true;
    }

    private boolean isSuitableForWall(boolean[][] map, int x, int y) {
        // Check if each side has at least 3 empty tiles
        return isSideSuitable(map, x, y, -1, 0) && // Check left side
                isSideSuitable(map, x, y, 1, 0) &&  // Check right side
                isSideSuitable(map, x, y, 0, -1) && // Check top side
                isSideSuitable(map, x, y, 0, 1);    // Check bottom side
    }

    private boolean isSideSuitable(boolean[][] map, int x, int y, int dx, int dy) {
        try{
            int emptyTiles = 0;
            while(map[y + dy][x + dx]){
                x += dx;
                y += dy;
            }
            for(int i = 1; i <= 3; i++){
                if(!map[y + dy * i][x + dx * i]){
                    emptyTiles++;
                }
            }
            if(emptyTiles < 3){
                return false;
            }
        } catch (Exception e){
            return false;
        }
        return true;
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
