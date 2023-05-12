package cz.empatix.map.domain;

import cz.empatix.rooms.service.PathRoom;
import cz.empatix.rooms.service.Room;

public class PathRoomGenInfo extends RoomGenInfo{
    public final static int MIN_ROWS = 4;
    public final static int MIN_COLS = 4;
    @Override
    public void initDimensions() {
        rows = MIN_ROWS;
        cols = MIN_COLS;
    }

    @Override
    public Room constructRoom() {
        return new PathRoom(xOffset, yOffset, rows, cols);
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
