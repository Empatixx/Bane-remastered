package cz.empatix.roomobjects.suppliers;

import cz.empatix.roomobjects.domain.Banner;
import cz.empatix.roomobjects.domain.Flamethrower;
import cz.empatix.roomobjects.domain.RoomObject;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.concurrent.ThreadLocalRandom;

public class FlamethrowerSupplier implements RoomObjectSupplier<Flamethrower> {
    private final static int MAX_TRIES = 100;

    private final TileMap tileMap;
    private final Room room;

    public FlamethrowerSupplier(final Room room,
                                final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Flamethrower supply() {
        if (!isPossible()) {
            return null;
        }
        while (true) {
            final int cols = tileMap.getTotalCollumns();
            final int rows = tileMap.getTotalRows();

            final int xTile = ThreadLocalRandom.current().nextInt(3, cols - 2);
            final int yTile = ThreadLocalRandom.current().nextInt(3, rows - 2);

            if (tileMap.isWall(xTile, yTile)) {
                if (yTile + 1 < rows && !tileMap.isWall(xTile, yTile + 1)) {
                    final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                    final int yPos = room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

                    final Flamethrower flamethrower = new Flamethrower();
                    flamethrower.setPosition(xPos, yPos);
                    return flamethrower;
                }
            }
        }
    }
    private boolean isPossible(){
        final int cols = tileMap.getTotalCollumns();
        final int rows = tileMap.getTotalRows();

        for (int i = 3; i < rows - 2; i++){
            for (int j = 3; j < cols - 2; j++) {
                if (tileMap.isWall(j, i)) {
                    if (i + 1 < rows && !tileMap.isWall(j, i + 1)) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
