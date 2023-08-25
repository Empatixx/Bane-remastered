package cz.empatix.map.domain;

import cz.empatix.items.service.ItemService;
import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.TreasureRoom;

public class TreasureRoomGenInfo extends RoomGenInfo {
    private final ItemService itemService;
    public TreasureRoomGenInfo(final ItemService itemService) {
        this.itemService = itemService;
    }
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new TreasureRoom(rows, cols, xOffset, yOffset, neighbours.keySet(), itemService);
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
