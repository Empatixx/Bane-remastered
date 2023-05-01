package cz.empatix.map.domain;

import cz.empatix.rooms.service.PathRoom;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.tilemaps.PathTileMap;

public class PathRoomGenInfo extends RoomGenInfo{
    @Override
    public void initDimensions() {
        cols = PathTileMap.COLS;
        rows = PathTileMap.ROWS;
    }

    @Override
    public Room constructRoom() {
        return new PathRoom(xOffset, yOffset);
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
