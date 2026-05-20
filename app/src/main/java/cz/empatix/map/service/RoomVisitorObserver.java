package cz.empatix.map.service;

import cz.empatix.rooms.service.Room;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

public class RoomVisitorObserver {
    private final List<Consumer<Room>> listeners;

    public RoomVisitorObserver() {
        this.listeners = new ArrayList<>();
    }

    public void addListener(final Consumer<Room> listener) {
        listeners.add(listener);
    }

    public void observe(final Room room) {
        listeners.forEach(listener -> listener.accept(room));
    }
}
