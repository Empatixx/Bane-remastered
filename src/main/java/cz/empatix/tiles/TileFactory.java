package cz.empatix.tiles;

import cz.empatix.tiles.domain.Tile;

public class TileFactory {
    private static final int[] TOP_WALL = {22, 23, 24, 25};
    private static final int[] BOTTOM_WALL = {30, 31};
    private static final int[] LEFT_WALL = {27, 39};
    private static final int[] RIGHT_WALL = {28, 40};
    private static final int[] WALL = {41};

    private static final int[] BOTTOM_LEFT_OUTER_CORNER = {38};
    private static final int[] BOTTOM_RIGHT_OUTE_CORNER = {35};
    private static final int[] TOP_LEFT_OUTER_CORNER = TOP_WALL;
    private static final int[] TOP_RIGHT_OUTER_CORNER = TOP_WALL;

    private static final int[] BOTTOM_RIGHT_INNER_CORNER = {29};
    private static final int[] BOTTOM_LEFT_INNER_CORNER = {34};
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

    public Tile[][] createTileMap(final boolean[][] map) {
        final Tile[][] tileMap = new Tile[map.length][map[0].length];
        for(int x = 0;x< map.length;x++) {
            for (int y = 0; y < map[x].length; y++) {
                final boolean isCollision = isCollision(x, y, map);

                final boolean topCollision = isCollision(x, y - 1, map);
                final boolean bottomCollision = isCollision(x, y + 1, map);
                final boolean leftCollision = isCollision(x - 1, y, map);
                final boolean rightCollision = isCollision(x + 1, y, map);

                final boolean topLeftCollision = isCollision(x - 1, y - 1, map);
                final boolean topRightCollision = isCollision(x + 1, y - 1, map);
                final boolean bottomLeftCollision = isCollision(x - 1, y + 1, map);
                final boolean bottomRightCollision = isCollision(x + 1, y + 1, map);

                final Tile tile = createTile();
                tileMap[x][y] = tile;
            }
        }
        return tileMap;
    }
    public boolean isCollision(final int x, final int y, boolean[][] map){
        try{
            return map[x][y];
        } catch (ArrayIndexOutOfBoundsException e){
            return true;
        }
    }
    public Tile createTile(){
        return null;
    }
}
