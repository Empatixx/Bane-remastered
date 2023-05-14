package cz.empatix.map.domain;

import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.TreasureRoom;

public class TreasureRoomGenInfo extends RoomGenInfo{
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new TreasureRoom(rows, cols, xOffset, yOffset, getDirections());
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
