package cz.empatix.map.domain;

import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.death.service.VictoryService;
import cz.empatix.enemies.service.DifficultyService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.service.MapLoaderService;
import cz.empatix.player.CoinService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.BossRoom;
import cz.empatix.rooms.service.Room;

public class BossRoomGenInfo extends RoomGenInfo {
    private final ItemService itemService;
    private final MapLoaderService mapLoaderService;
    private final StatisticsService statisticsService;
    private final CoinService coinService;
    private final DifficultyService difficultyService;
    private final RngService rngService;
    private final VictoryService victoryService;

    public BossRoomGenInfo(final ItemService itemService,
                           final MapLoaderService mapLoaderService,
                           final StatisticsService statisticsService,
                           final CoinService coinService,
                           final DifficultyService difficultyService,
                           final RngService rngService,
                           final VictoryService victoryService) {
        this.itemService = itemService;
        this.mapLoaderService = mapLoaderService;
        this.statisticsService = statisticsService;
        this.coinService = coinService;
        this.difficultyService = difficultyService;
        this.rngService = rngService;
        this.victoryService = victoryService;
    }

    @Override
    public void initDimensions() {
        rows = 15;
        cols = 20;
    }

    @Override
    public Room constructRoom() {
        return new BossRoom(itemService, mapLoaderService, rows, cols, xOffset, yOffset, neighbours.keySet(),
                statisticsService, coinService, difficultyService, rngService, victoryService);
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
