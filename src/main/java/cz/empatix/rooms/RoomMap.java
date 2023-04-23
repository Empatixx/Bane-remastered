package cz.empatix.rooms;

import cz.empatix.render.service.Camera;
import cz.empatix.render.attributes.Drawable;

import java.util.ArrayList;
import java.util.List;

public class RoomMap {
    private final static int TOTAL_NORMAL_ROOMS = 5;
    private final static int TOTAL_BOSS_ROOMS = 1;
    private final static int TOTAL_SHOP_ROOMS = 1;
    private final static int TOTAL_START_ROOMS = 1;
    private final static int TOTAL_ROOMS = TOTAL_NORMAL_ROOMS + TOTAL_BOSS_ROOMS + TOTAL_SHOP_ROOMS + TOTAL_START_ROOMS;

    private final List<Room> rooms;
    public RoomMap(){
        rooms = new ArrayList<>(TOTAL_ROOMS);
        rooms.add(new CommonRoom());
        rooms.forEach(Room::init);

        Camera.setMinDimensions(0,0);
        Camera.setMaxDimensions(rooms.get(0).getWidth(), rooms.get(0).getHeight());
    }

    public List<Drawable> toRender(double dt) {
        return rooms.stream()
                .map(Room::toRender)
                .flatMap(List::stream)
                .toList();
    }
}
