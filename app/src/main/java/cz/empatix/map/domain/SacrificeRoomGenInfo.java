package cz.empatix.map.domain;

import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.service.ItemService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.SacrificeRoom;
import cz.empatix.rooms.service.ShopRoom;

public class SacrificeRoomGenInfo extends RoomGenInfo{
    private final ItemService itemService;
    private final StatisticsService statisticsService;
    private final RngService rngService;
    public SacrificeRoomGenInfo(final ItemService itemService,
                           final StatisticsService statisticsService,
                           final RngService rngService) {
        this.itemService = itemService;
        this.statisticsService = statisticsService;
        this.rngService = rngService;
    }
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new SacrificeRoom(itemService, rows, cols, xOffset, yOffset, neighbours.keySet(),
                statisticsService, rngService);
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.SACRIFICE;
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
