package cz.empatix.map.domain;

import cz.empatix.items.service.ItemService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.BossRoom;
import cz.empatix.rooms.service.Room;

public class BossRoomGenInfo extends RoomGenInfo {
    private final ItemService itemService;

    public BossRoomGenInfo(final ItemService itemService) {
        this.itemService = itemService;
    }

    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new BossRoom(itemService, rows, cols, xOffset, yOffset, neighbours.keySet());
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.BOSS;
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
