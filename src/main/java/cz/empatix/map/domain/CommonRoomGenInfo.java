package cz.empatix.map.domain;

import cz.empatix.rooms.service.CommonRoom;
import cz.empatix.rooms.service.Room;

public class CommonRoomGenInfo extends RoomGenInfo{
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
       return new CommonRoom(rows, cols, xOffset, yOffset,  neighbours.keySet());
    }

    @Override
    public boolean canCreateNeighbours() {
        return neighbours.size() < MAX_ROOM_NEIGHBOURS;
    }
}
