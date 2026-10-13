package cz.empatix.map.domain;

import cz.empatix.rooms.service.Room;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertSame;

class MapGenerationResultTest {

    @Test
    void accessorsReturnConstructorArguments() {
        final List<Room> rooms = List.of();
        final List<Room> paths = List.of();

        // Room is not constructed here (would require heavy game deps); null/empty is enough
        final MapGenerationResult result = new MapGenerationResult(null, rooms, paths);

        assertNull(result.startRoom());
        assertSame(rooms, result.rooms());
        assertSame(paths, result.paths());
    }
}
