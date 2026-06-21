package cz.empatix.tiles.tilemaps;

import cz.empatix.common.RngService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.factory.CarpetTileFactory;
import cz.empatix.tiles.service.PerlinNoiseGenerator;

import java.util.Arrays;
import java.util.List;
import java.util.Set;

public class ProceduralCarpetTileMap implements TileMap {
    private final Tile[][] tiles;
    private final int rows;
    private final int cols;
    private final int xOffset;
    private final int yOffset;

    public ProceduralCarpetTileMap(final int rows, final int cols,
                                   final int xOffset, final int yOffset,
                                   final Set<RoomDirection> directions) {
        this.rows = rows;
        this.cols = cols;
        this.xOffset = xOffset;
        this.yOffset = yOffset;

        final long tileSeed = RngService.seedForCoordinate(xOffset, yOffset);
        final CarpetTileFactory tileFactory = new CarpetTileFactory(new RngService(tileSeed));

        final boolean[][] map = new boolean[rows][cols];
        applyPerlinNoiseToMap(map, 0.05, 0.5, 1, tileSeed);
        postProcessMap(map);
        final Tile[][] carpetTiles = tileFactory.createTileMap(map, xOffset, yOffset);
        tiles = carpetTiles;
    }

    // min 2x2 carpets, also make carpets grouped, do not make empty places between them, so they are connected
    // or create a fragments of carpets  if there are too distant from each other
    private void postProcessMap(final boolean[][] map) {

    }

    private void applyPerlinNoiseToMap(boolean[][] map, final double scale, final double lowerThreshold, final double upperThreshold, final long seed) {
        final PerlinNoiseGenerator perlinNoiseGenerator = new PerlinNoiseGenerator(seed);

        for (int y = 0; y < map.length; y++) {
            for (int x = 0; x < map[0].length; x++) {
                double noiseValue = perlinNoiseGenerator.noise(x * scale, y * scale);
                if (noiseValue > lowerThreshold && noiseValue < upperThreshold) {
                    map[y][x] = true; // Set to wall
                }
            }
        }
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
