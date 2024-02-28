package cz.empatix.map.service;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.rooms.domain.RoomEvent;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.service.Room;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;
import java.util.stream.Collectors;

public class RoomEventObserver {
    private final List<Consumer<RoomEvent>> listeners;
    private Map<Room, RoomState> lastState;

    public RoomEventObserver() {
        this.listeners = new ArrayList<>();
    }

    public void apply(final MapGenerationResult result) {
        this.lastState = result.rooms()
                .stream()
                .collect(Collectors.toMap(room -> room, Room::getState));

    }

    public void observe() {
        lastState.keySet().forEach(room -> {
            final RoomState currentState = room.getState();
            final RoomState lastRoomState = lastState.get(room);
            if (currentState != lastRoomState) {
                final RoomEvent event = new RoomEvent(room.getState(), lastRoomState, room.getRoomType());
                listeners.forEach(listener -> listener.accept(event));
                lastState.put(room, currentState);
            }
        });
    }

    public void addListener(final Consumer<RoomEvent> listener) {
        listeners.add(listener);
    }
}
