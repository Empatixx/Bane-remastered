package cz.empatix.map.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.rooms.service.Room;

import java.util.List;

public class MapService {
    private final List<Room> rooms;
    private final List<Room> pathRooms;

    public MapService(final List<Room> rooms,
                      final List<Room> pathRooms) {
        this.pathRooms = pathRooms;
        this.rooms = rooms;
    }

    public List<Drawable> toRender(double dt) {
        return rooms.stream()
                .map(Room::toRender)
                .flatMap(List::stream)
                .toList();
    }
}
