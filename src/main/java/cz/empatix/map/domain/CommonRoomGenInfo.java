package cz.empatix.map.domain;

import cz.empatix.items.service.ItemService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.CommonRoom;
import cz.empatix.rooms.service.Room;

public class CommonRoomGenInfo extends RoomGenInfo{
    private final ItemService itemService;
    public CommonRoomGenInfo(final ItemService itemService){
        this.itemService = itemService;
    }
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
       return new CommonRoom(itemService, rows, cols, xOffset, yOffset,  neighbours.keySet());
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.NORMAL;
    }

    @Override
    public boolean canCreateNeighbours() {
        return neighbours.size() < MAX_ROOM_NEIGHBOURS;
    }
}
