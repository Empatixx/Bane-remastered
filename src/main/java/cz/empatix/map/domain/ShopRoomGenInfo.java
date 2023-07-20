package cz.empatix.map.domain;

import cz.empatix.items.service.ItemService;
import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.ShopRoom;

public class ShopRoomGenInfo extends RoomGenInfo {
    private final ItemService itemService;
    public ShopRoomGenInfo(final ItemService itemService){
        this.itemService = itemService;
    }
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new ShopRoom(itemService, rows, cols, xOffset, yOffset, neighbours.keySet());
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
