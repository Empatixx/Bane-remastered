package cz.empatix.map.domain;

import cz.empatix.rooms.service.BossRoom;
import cz.empatix.rooms.service.Room;

public class BossRoomGenInfo extends RoomGenInfo{
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new BossRoom(rows, cols, xOffset, yOffset, neighbours.keySet());
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
