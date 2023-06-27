package cz.empatix.map.domain;

import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.ShopRoom;

public class ShopRoomGenInfo extends RoomGenInfo {
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new ShopRoom(rows, cols, xOffset, yOffset, neighbours.keySet());
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
