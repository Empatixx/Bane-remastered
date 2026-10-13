package cz.empatix.tiles.factory;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.tiles.domain.CarpetTile;
import cz.empatix.tiles.domain.EmptyTile;
import cz.empatix.tiles.domain.Tile;

import java.util.EnumMap;
import java.util.List;
import java.util.Map;


public class CarpetTileFactory {
    private final cz.empatix.common.RngService rng;
    private final Map<TileOrientation, List<Integer>> tileNumbers;
    private static final String TEXTURE_NAME = "ingame/carpet_tilemap.tga";

    public CarpetTileFactory(final cz.empatix.common.RngService rng) {
        this.rng = rng;
        tileNumbers = new EnumMap<TileOrientation, List<Integer>>(TileOrientation.class);
        tileNumbers.put(TileOrientation.TOP_LEFT_OUTER_CORNER, List.of(0));
        tileNumbers.put(TileOrientation.TOP, List.of(1, 2));
        tileNumbers.put(TileOrientation.TOP_RIGHT_OUTER_CORNER, List.of(3));
        tileNumbers.put(TileOrientation.LEFT, List.of(4, 7));
        tileNumbers.put(TileOrientation.CENTER, List.of(5));
        tileNumbers.put(TileOrientation.RIGHT, List.of(6, 8));
        tileNumbers.put(TileOrientation.BOTTOM_LEFT_OUTER_CORNER, List.of(9));
        tileNumbers.put(TileOrientation.BOTTOM, List.of(10, 11));
        tileNumbers.put(TileOrientation.BOTTOM_RIGHT_OUTER_CORNER, List.of(12));
    }

    public Tile[][] createTileMap(final boolean[][] map,
                                            final int xOffset,
                                            final int yOffset) {
        final Tile[][] tileMap = new Tile[map.length][map[0].length];
        for (int y = 0; y < map.length; y++) {
            for (int x = 0; x < map[y].length; x++) {
                final boolean isCollision = isCollision(x, y, map);

                final boolean topCollision = isCollision(x, y - 1, map);
                final boolean bottomCollision = isCollision(x, y + 1, map);
                final boolean leftCollision = isCollision(x - 1, y, map);
                final boolean rightCollision = isCollision(x + 1, y, map);

                final boolean topLeftCollision = isCollision(x - 1, y - 1, map);
                final boolean topRightCollision = isCollision(x + 1, y - 1, map);
                final boolean bottomLeftCollision = isCollision(x - 1, y + 1, map);
                final boolean bottomRightCollision = isCollision(x + 1, y + 1, map);

                final Tile tile = createTile(isCollision, topCollision, bottomCollision,
                        leftCollision, rightCollision, topLeftCollision, topRightCollision,
                        bottomLeftCollision, bottomRightCollision, x, y, xOffset, yOffset);
                tileMap[y][x] = tile;
            }
        }
        return tileMap;
    }

    private Tile createTile(boolean isCollision,
                            boolean topCollision,
                            boolean bottomCollision,
                            boolean leftCollision,
                            boolean rightCollision,
                            boolean topLeftCollision,
                            boolean topRightCollision,
                            boolean bottomLeftCollision,
                            boolean bottomRightCollision,
                            int x,
                            int y,
                            int xOffset,
                            int yOffset) {
        if (isCollision && topCollision && bottomCollision && leftCollision && rightCollision
                && topLeftCollision && topRightCollision && bottomLeftCollision && bottomRightCollision) {
            return createTile(tileNumbers.get(TileOrientation.CENTER), x, y, xOffset, yOffset);
        } else if (isCollision && topCollision && bottomCollision && rightCollision && !leftCollision) {
            return createTile(tileNumbers.get(TileOrientation.LEFT), x, y, xOffset, yOffset);
        } else if (isCollision && topCollision && bottomCollision && leftCollision && !rightCollision) {
            return createTile(tileNumbers.get(TileOrientation.RIGHT), x, y, xOffset, yOffset);
        } else if (isCollision && topCollision && rightCollision && leftCollision && !bottomCollision) {
            return createTile(tileNumbers.get(TileOrientation.BOTTOM), x, y, xOffset, yOffset);
        } else if (isCollision && bottomCollision && rightCollision && leftCollision && !topCollision) {
            return createTile(tileNumbers.get(TileOrientation.TOP), x, y, xOffset, yOffset);
        } else if (isCollision && !topCollision && !leftCollision && rightCollision && bottomCollision) {
            return createTile(tileNumbers.get(TileOrientation.TOP_LEFT_OUTER_CORNER), x, y, xOffset, yOffset);
        } else if (isCollision && !topCollision && leftCollision && !rightCollision && bottomCollision) {
            return createTile(tileNumbers.get(TileOrientation.TOP_RIGHT_OUTER_CORNER), x, y, xOffset, yOffset);
        } else if (isCollision && topCollision && !leftCollision && rightCollision && !bottomCollision) {
            return createTile(tileNumbers.get(TileOrientation.BOTTOM_LEFT_OUTER_CORNER), x, y, xOffset, yOffset);
        } else if (isCollision && topCollision && leftCollision && !rightCollision && !bottomCollision) {
            return createTile(tileNumbers.get(TileOrientation.BOTTOM_RIGHT_OUTER_CORNER), x, y, xOffset, yOffset);
        } else {
            return new EmptyTile();
        }
    }

    private boolean isCollision(final int x, final int y, boolean[][] map) {
        try {
            return map[y][x];
        } catch (ArrayIndexOutOfBoundsException e) {
            return true;
        }
    }


    private CarpetTile createTile(final List<Integer> tileNumbers,
                                  final int x,
                                  final int y,
                                  final int xOffset,
                                  final int yOffset) {
        final TextureAtlas.Entry texture = AssetsService.getTexture(TEXTURE_NAME);
        final int tileNumber = tileNumbers.get(rng.nextInt(tileNumbers.size()));
        return CarpetTile.from(tileNumber, x, y, xOffset, yOffset, texture);
    }
}
