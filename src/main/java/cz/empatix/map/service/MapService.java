package cz.empatix.map.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.rooms.service.Room;

import java.util.List;

public class MapService {
    private final List<Room> rooms;
    public MapService(final List<Room> rooms){
        this.rooms = rooms;
    }

    public List<Drawable> toRender(double dt) {
        return rooms.stream()
                .map(Room::toRender)
                .flatMap(List::stream)
                .toList();
    }
}
