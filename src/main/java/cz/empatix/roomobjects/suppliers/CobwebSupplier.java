package cz.empatix.roomobjects.suppliers;

import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.Cobweb;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.concurrent.ThreadLocalRandom;

public class CobwebSupplier implements RoomObjectSupplier<Cobweb>{
    private final Room room;
    private final TileMap tileMap;

    public CobwebSupplier(final Room room, final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }

    @Override
    public Cobweb supply() {
        while (true) {
            final int cols = tileMap.getTotalCollumns();
            final int rows = tileMap.getTotalRows();

            final int xTile = ThreadLocalRandom.current().nextInt(0, cols - 1);
            final int yTile = ThreadLocalRandom.current().nextInt(0, rows - 1);

            if (!tileMap.isWall(xTile, yTile)) {
                if (xTile + 1 < cols && tileMap.isWall(xTile + 1, yTile)) {

                    final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                    final int yPos =  room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

                    final Cobweb cobweb = new Cobweb();
                    cobweb.setPosition(xPos, yPos);
                    cobweb.setState(AnimationState.TYPE1);
                    return cobweb;

                } else if (xTile - 1 >= 0 && tileMap.isWall(xTile - 1, yTile)) {
                    final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                    final int yPos =  room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

                    final Cobweb cobweb = new Cobweb();
                    cobweb.setPosition(xPos, yPos);
                    cobweb.setState(AnimationState.TYPE0);
                    return cobweb;

                }
            }
        }
    }
}
