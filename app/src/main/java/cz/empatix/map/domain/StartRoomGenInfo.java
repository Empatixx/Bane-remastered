package cz.empatix.map.domain;

import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.StartRoom;

public class StartRoomGenInfo extends RoomGenInfo {
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new StartRoom(rows, cols, xOffset, yOffset,  neighbours.keySet());
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.START;
    }

    @Override
    public boolean canCreateNeighbours() {
        return neighbours.size() < MAX_ROOM_NEIGHBOURS;
    }
}
