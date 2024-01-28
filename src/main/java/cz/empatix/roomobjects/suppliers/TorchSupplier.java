package cz.empatix.roomobjects.suppliers;

import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.Torch;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.concurrent.ThreadLocalRandom;

public class TorchSupplier implements RoomObjectSupplier<Torch>{
    private final Room room;
    private final TileMap tileMap;

    public TorchSupplier(final Room room, final TileMap tileMap) {
        this.room = room;
        this.tileMap = tileMap;
    }
    @Override
    public Torch supply() {
        while (true) {
            final int cols = tileMap.getTotalCollumns();
            final int rows = tileMap.getTotalRows();

            final int xTile = ThreadLocalRandom.current().nextInt(1, cols - 1);
            final int yTile = ThreadLocalRandom.current().nextInt(1, rows - 1);

            if (!tileMap.isWall(xTile, yTile)) {
                if (xTile + 1 < cols && tileMap.isWall(xTile + 1, yTile)) {

                    final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                    final int yPos =  room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

                    final Torch torch = new Torch();
                    torch.setPosition(xPos, yPos);
                    torch.setState(AnimationState.RIGHT_SIDE);
                    return torch;

                } else if (xTile - 1 >= 0 && tileMap.isWall(xTile - 1, yTile)) {
                    final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                    final int yPos =  room.getY() + yTile * Tile.TILE_SIZE * Tile.SCALE;

                    final Torch torch = new Torch();
                    torch.setPosition(xPos, yPos);
                    torch.setState(AnimationState.LEFT_SIDE);
                    return torch;

                } else if (yTile - 1 >= 0 && tileMap.isWall(xTile, yTile - 1)) {
                    final int xPos = room.getX() + xTile * Tile.TILE_SIZE * Tile.SCALE;
                    final int yPos = room.getY()  + (yTile - 1) * Tile.TILE_SIZE * Tile.SCALE;

                    final Torch torch = new Torch();
                    torch.setPosition(xPos, yPos);
                    torch.setState(AnimationState.TOP);
                    return torch;
                }
            }
        }
    }
}
