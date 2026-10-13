package cz.empatix.map.service;

import cz.empatix.rooms.service.Room;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;

/** Covers the listener fan-out of {@link RoomVisitorObserver} (pure, no assets). */
class RoomVisitorObserverTest {

    @Test
    void observeNotifiesEveryRegisteredListenerWithTheRoom() {
        final RoomVisitorObserver observer = new RoomVisitorObserver();
        final List<Room> seenA = new ArrayList<>();
        final List<Room> seenB = new ArrayList<>();
        observer.addListener(seenA::add);
        observer.addListener(seenB::add);

        final Room room = mock(Room.class);
        observer.observe(room);

        assertEquals(1, seenA.size());
        assertSame(room, seenA.get(0));
        assertSame(room, seenB.get(0));
    }

    @Test
    void observeWithoutListenersIsANoOp() {
        final RoomVisitorObserver observer = new RoomVisitorObserver();

        // Must not throw with no listeners attached.
        observer.observe(mock(Room.class));
    }

    @Test
    void eachObserveCallNotifiesAgain() {
        final RoomVisitorObserver observer = new RoomVisitorObserver();
        final List<Room> seen = new ArrayList<>();
        observer.addListener(seen::add);

        final Room first = mock(Room.class);
        final Room second = mock(Room.class);
        observer.observe(first);
        observer.observe(second);

        assertEquals(List.of(first, second), seen);
        assertTrue(seen.contains(second));
    }
}
