package cz.empatix.map.service;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.rooms.domain.RoomEvent;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/** Covers the state-diffing event emission of {@link RoomEventObserver} (pure, no assets). */
class RoomEventObserverTest {

    private MapGenerationResult resultWith(final Room... rooms) {
        return new MapGenerationResult(rooms[0], List.of(rooms), List.of());
    }

    @Test
    void noEventWhenStateIsUnchanged() {
        final Room room = mock(Room.class);
        when(room.getState()).thenReturn(RoomState.OPEN);
        when(room.getRoomType()).thenReturn(RoomType.NORMAL);

        final RoomEventObserver observer = new RoomEventObserver();
        final List<RoomEvent> events = new ArrayList<>();
        observer.addListener(events::add);
        observer.apply(resultWith(room));

        observer.observe();

        assertTrue(events.isEmpty());
    }

    @Test
    void emitsEventWithOldAndNewStateOnChange() {
        final Room room = mock(Room.class);
        when(room.getState()).thenReturn(RoomState.OPEN);
        when(room.getRoomType()).thenReturn(RoomType.BOSS);

        final RoomEventObserver observer = new RoomEventObserver();
        final List<RoomEvent> events = new ArrayList<>();
        observer.addListener(events::add);
        observer.apply(resultWith(room)); // snapshot: OPEN

        when(room.getState()).thenReturn(RoomState.CLOSED);
        observer.observe();

        assertEquals(1, events.size());
        final RoomEvent event = events.get(0);
        assertSame(RoomState.CLOSED, event.state());
        assertSame(RoomState.OPEN, event.lastState());
        assertSame(RoomType.BOSS, event.roomType());
    }

    @Test
    void stateChangeIsReportedOnlyOnce() {
        final Room room = mock(Room.class);
        when(room.getState()).thenReturn(RoomState.OPEN);
        when(room.getRoomType()).thenReturn(RoomType.SHOP);

        final RoomEventObserver observer = new RoomEventObserver();
        final List<RoomEvent> events = new ArrayList<>();
        observer.addListener(events::add);
        observer.apply(resultWith(room));

        when(room.getState()).thenReturn(RoomState.CLOSED);
        observer.observe();
        // Second observe with the same (now-current) state must not re-fire.
        observer.observe();

        assertEquals(1, events.size());
    }
}
