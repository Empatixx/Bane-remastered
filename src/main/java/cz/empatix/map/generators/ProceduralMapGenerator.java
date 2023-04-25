package cz.empatix.map.generators;

import cz.empatix.map.service.MapService;
import cz.empatix.rooms.service.*;

import java.util.ArrayList;
import java.util.List;

public class ProceduralMapGenerator implements MapGenerator{
    private final static int TOTAL_NORMAL_ROOMS = 5;
    private final static int TOTAL_BOSS_ROOMS = 1;
    private final static int TOTAL_SHOP_ROOMS = 1;
    private final static int TOTAL_START_ROOMS = 1;
    private final static int TOTAL_ROOMS = TOTAL_NORMAL_ROOMS + TOTAL_BOSS_ROOMS + TOTAL_SHOP_ROOMS + TOTAL_START_ROOMS;

    @Override
    public MapService generate() {
        final List<Room> rooms = generateRooms();

        return new MapService(rooms);
    }
    private List<Room> generateRooms(){
        final List<Room> rooms = new ArrayList<>(TOTAL_ROOMS);
        for(int i = 0; i < TOTAL_NORMAL_ROOMS; i++){
            rooms.add(new CommonRoom());
        }
        for(int i = 0; i < TOTAL_BOSS_ROOMS; i++){
            rooms.add(new BossRoom());
        }
        for(int i = 0; i < TOTAL_SHOP_ROOMS; i++){
            rooms.add(new ShopRoom());
        }
        for(int i = 0; i < TOTAL_START_ROOMS; i++){
            rooms.add(new StartRoom());
        }
        return List.copyOf(rooms);
    }
}
