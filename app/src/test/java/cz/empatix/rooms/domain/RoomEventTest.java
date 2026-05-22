package cz.empatix.rooms.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;

class RoomEventTest {

    @Test
    void accessorsReturnConstructorArguments() {
        final RoomEvent event = new RoomEvent(RoomState.OPEN, RoomState.CLOSED, RoomType.BOSS);

        assertEquals(RoomState.OPEN, event.state());
        assertEquals(RoomState.CLOSED, event.lastState());
        assertEquals(RoomType.BOSS, event.roomType());
    }

    @Test
    void recordEqualityAndHashCode() {
        final RoomEvent a = new RoomEvent(RoomState.CLOSED, RoomState.OPEN, RoomType.SHOP);
        final RoomEvent b = new RoomEvent(RoomState.CLOSED, RoomState.OPEN, RoomType.SHOP);
        final RoomEvent c = new RoomEvent(RoomState.OPEN, RoomState.OPEN, RoomType.SHOP);

        assertEquals(a, b);
        assertEquals(a.hashCode(), b.hashCode());
        assertNotEquals(a, c);
    }
}
