package cz.empatix.map.generators;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.service.CommonRoom;

import java.util.List;
import java.util.Set;

public class StaticMapGenerator implements MapGenerator {

    @Override
    public MapGenerationResult generate() {
        final CommonRoom commonRoom = new CommonRoom(20, 20, 0, 0, Set.of());
        commonRoom.init();
        Camera.setMinDimensions(0, 0);
        Camera.setMaxDimensions(commonRoom.getWidth(), commonRoom.getHeight());
        return new MapGenerationResult(List.of(commonRoom), List.of());
    }
}
