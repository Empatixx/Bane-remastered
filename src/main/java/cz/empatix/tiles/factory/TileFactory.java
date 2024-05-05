package cz.empatix.tiles.factory;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.domain.TileImpl;

import java.util.concurrent.ThreadLocalRandom;

public class TileFactory {
    private final static int TILE_SIZE = TileImpl.SCALE * TileImpl.TILE_SIZE;
    private final static String TEXTURE_NAME = "ingame/tileset.tga";
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private static final int[] TOP_WALL = {22, 23, 24, 25};
    private static final int[] BOTTOM_WALL = {30, 31};
    private static final int[] LEFT_WALL = {27, 39};
    private static final int[] RIGHT_WALL = {28, 40};
    private static final int[] WALL = {41};

    private static final int[] BOTTOM_LEFT_OUTER_CORNER = TOP_WALL;
    private static final int[] BOTTOM_RIGHT_OUTER_CORNER = TOP_WALL;
    private static final int[] TOP_LEFT_OUTER_CORNER = {35};
    private static final int[] TOP_RIGHT_OUTER_CORNER = {38};

    private static final int[] BOTTOM_RIGHT_INNER_CORNER = {34};
    private static final int[] BOTTOM_LEFT_INNER_CORNER = {29};
    private static final int[] TOP_RIGHT_INNER_CORNER = {26};
    private static final int[] TOP_LEFT_INNER_CORNER = {21};

    private static final int[] FLOOR_NEAR_LEFT_WALL = {4};
    private static final int[] FLOOR_NEAR_RIGHT_WALL = {7};
    private static final int[] FLOOR_NEAR_TOP_WALL = {1, 2};
    private static final int[] FLOOR_NEAR_BOTTOM_WALL = {9, 10};

    private static final int[] FLOOR_NEAR_LEFT_TOP_CORNER = {0};
    private static final int[] FLOOR_NEAR_RIGHT_TOP_CORNER = {3};
    private static final int[] FLOOR_NEAR_LEFT_BOTTOM_CORNER = {8};
    private static final int[] FLOOR_NEAR_RIGHT_BOTTOM_CORNER = {11};

    private static final int[] FLOOR = {5, 6, 6, 6, 6, 6, 12, 13, 14, 15, 16, 17, 18};

    public Tile[][] createTilemapFromClamp(final boolean[][] map,
                                               final int xOffset,
                                               final int yOffset) {
        final Tile[][] tileMap = new TileImpl[map.length - 2][map[0].length - 2];
        for (int y = 1; y < map.length - 1; y++) {
            for (int x = 1; x < map[y].length - 1; x++) {
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
                        bottomLeftCollision, bottomRightCollision, x - 1, y - 1, xOffset, yOffset);
                tileMap[y - 1][x - 1] = tile;
            }
        }
        return tileMap;
    }

    public Tile[][] createTileMap(final boolean[][] map, final int xOffset, final int yOffset) {
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

    private boolean isCollision(final int x, final int y, boolean[][] map) {
        try {
            return map[y][x];
        } catch (ArrayIndexOutOfBoundsException e) {
            return true;
        }
    }

    private Tile createTile(final boolean current,
                                final boolean top,
                                final boolean bottom,
                                final boolean left,
                                final boolean right,
                                final boolean topLeft,
                                final boolean topRight,
                                final boolean bottomLeft,
                                final boolean bottomRight,
                                final int x,
                                final int y,
                                final int xOffset,
                                final int yOffset) {
        if (current && top && bottom && right && left && topLeft && topRight && bottomLeft && bottomRight) {
            return createTile(WALL[RANDOM.nextInt(WALL.length)], x, y, xOffset, yOffset);
        } else if (current && top && bottom && right && !left) {
            return createTile(RIGHT_WALL[RANDOM.nextInt(RIGHT_WALL.length)], x, y, xOffset, yOffset);
        } else if (current && top && bottom && left && !right) {
            return createTile(LEFT_WALL[RANDOM.nextInt(LEFT_WALL.length)], x, y, xOffset, yOffset);
        } else if (current && top && right && left && !bottom) {
            return createTile(TOP_WALL[RANDOM.nextInt(TOP_WALL.length)], x, y, xOffset, yOffset);
        } else if (current && bottom && right && left && !top) {
            return createTile(BOTTOM_WALL[RANDOM.nextInt(BOTTOM_WALL.length)], x, y, xOffset, yOffset);
        } else if (current && bottom && left && !bottomLeft) {
            return createTile(TOP_RIGHT_INNER_CORNER[RANDOM.nextInt(TOP_RIGHT_INNER_CORNER.length)], x, y, xOffset, yOffset);
        } else if (current && bottom && right && !bottomRight) {
            return createTile(TOP_LEFT_INNER_CORNER[RANDOM.nextInt(TOP_LEFT_INNER_CORNER.length)], x, y, xOffset, yOffset);
        } else if (current && top && left && !topLeft) {
            return createTile(BOTTOM_RIGHT_INNER_CORNER[RANDOM.nextInt(BOTTOM_RIGHT_INNER_CORNER.length)], x, y, xOffset, yOffset);
        } else if (current && top && right && !topRight) {
            return createTile(BOTTOM_LEFT_INNER_CORNER[RANDOM.nextInt(BOTTOM_LEFT_INNER_CORNER.length)], x, y, xOffset, yOffset);
        } else if (current && !top && !left && right && bottom) {
            return createTile(TOP_LEFT_OUTER_CORNER[RANDOM.nextInt(TOP_LEFT_OUTER_CORNER.length)], x, y, xOffset, yOffset);
        } else if (current && !top && left && !right && bottom) {
            return createTile(TOP_RIGHT_OUTER_CORNER[RANDOM.nextInt(TOP_RIGHT_OUTER_CORNER.length)], x, y, xOffset, yOffset);
        } else if (current && top && !left && right && !bottom) {
            return createTile(BOTTOM_LEFT_OUTER_CORNER[RANDOM.nextInt(BOTTOM_LEFT_OUTER_CORNER.length)], x, y, xOffset, yOffset);
        } else if (current && top && left && !right && !bottom) {
            return createTile(BOTTOM_RIGHT_OUTER_CORNER[RANDOM.nextInt(BOTTOM_RIGHT_OUTER_CORNER.length)], x, y, xOffset, yOffset);
        } else if (!current && top && right) {
            return createTile(FLOOR_NEAR_RIGHT_TOP_CORNER[RANDOM.nextInt(FLOOR_NEAR_RIGHT_TOP_CORNER.length)], x, y, xOffset, yOffset);
        } else if (!current && top && left) {
            return createTile(FLOOR_NEAR_LEFT_TOP_CORNER[RANDOM.nextInt(FLOOR_NEAR_LEFT_TOP_CORNER.length)], x, y, xOffset, yOffset);
        } else if (!current && bottom && right) {
            return createTile(FLOOR_NEAR_RIGHT_BOTTOM_CORNER[RANDOM.nextInt(FLOOR_NEAR_RIGHT_BOTTOM_CORNER.length)], x, y, xOffset, yOffset);
        } else if (!current && bottom && left) {
            return createTile(FLOOR_NEAR_LEFT_BOTTOM_CORNER[RANDOM.nextInt(FLOOR_NEAR_LEFT_BOTTOM_CORNER.length)], x, y, xOffset, yOffset);
        } else if (!current && bottom) {
            return createTile(FLOOR_NEAR_BOTTOM_WALL[RANDOM.nextInt(FLOOR_NEAR_BOTTOM_WALL.length)], x, y, xOffset, yOffset);
        } else if (!current && top) {
            return createTile(FLOOR_NEAR_TOP_WALL[RANDOM.nextInt(FLOOR_NEAR_TOP_WALL.length)], x, y, xOffset, yOffset);
        } else if (!current && left) {
            return createTile(FLOOR_NEAR_LEFT_WALL[RANDOM.nextInt(FLOOR_NEAR_LEFT_WALL.length)], x, y, xOffset, yOffset);
        } else if (!current && right) {
            return createTile(FLOOR_NEAR_RIGHT_WALL[RANDOM.nextInt(FLOOR_NEAR_RIGHT_WALL.length)], x, y, xOffset, yOffset);
        } else {
            return createTile(FLOOR[RANDOM.nextInt(FLOOR.length)], x, y, xOffset, yOffset);
        }
    }

    private Tile createTile(final int tileNumber,
                            final int x,
                            final int y,
                            final int xOffset,
                            final int yOffset) {
        final TextureAtlas.Entry texture = AssetsService.getTexture(TEXTURE_NAME);
        return TileImpl.from(tileNumber, x, y, xOffset, yOffset, texture);
    }
}