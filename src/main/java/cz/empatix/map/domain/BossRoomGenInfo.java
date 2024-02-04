package cz.empatix.map.domain;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.service.MapLoaderService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.BossRoom;
import cz.empatix.rooms.service.Room;

public class BossRoomGenInfo extends RoomGenInfo {
    private final ItemService itemService;
    private final MapLoaderService mapLoaderService;
    private final StatisticsService statisticsService;

    public BossRoomGenInfo(final ItemService itemService,
                           final MapLoaderService mapLoaderService,
                           final StatisticsService statisticsService) {
        this.itemService = itemService;
        this.mapLoaderService = mapLoaderService;
        this.statisticsService = statisticsService;
    }

    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new BossRoom(itemService, mapLoaderService, rows, cols, xOffset, yOffset, neighbours.keySet(),
                statisticsService);
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
