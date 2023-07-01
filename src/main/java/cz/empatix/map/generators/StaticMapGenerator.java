package cz.empatix.map.generators;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.rooms.service.LobbyRoom;

import java.util.List;

public class StaticMapGenerator implements MapGenerator {

    @Override
    public MapGenerationResult generate() {
        final LobbyRoom lobbyRoom = new LobbyRoom();
        lobbyRoom.init();
        return new MapGenerationResult(lobbyRoom, List.of(lobbyRoom), List.of());
    }
}
