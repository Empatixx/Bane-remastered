package cz.empatix.map.domain;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.service.ItemService;
import cz.empatix.player.CoinService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.ShopRoom;

public class ShopRoomGenInfo extends RoomGenInfo {
    private final ItemService itemService;
    private final StatisticsService statisticsService;
    private final CoinService coinService;
    public ShopRoomGenInfo(final ItemService itemService,
                           final StatisticsService statisticsService,
                           final CoinService coinService) {
        this.itemService = itemService;
        this.statisticsService = statisticsService;
        this.coinService = coinService;
    }
    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new ShopRoom(itemService, rows, cols, xOffset, yOffset, neighbours.keySet(),
                statisticsService, coinService);
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.SHOP;
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
