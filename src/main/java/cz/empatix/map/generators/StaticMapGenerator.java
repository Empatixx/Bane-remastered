package cz.empatix.map.generators;

import cz.empatix.map.service.MapService;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.service.CommonRoom;

import java.util.List;

public class StaticMapGenerator implements MapGenerator {

    @Override
    public MapService generate() {
        final CommonRoom commonRoom = new CommonRoom(20, 20, 0, 0, List.of());
        commonRoom.init();
        Camera.setMinDimensions(0, 0);
        Camera.setMaxDimensions(commonRoom.getWidth(), commonRoom.getHeight());
        return new MapService(List.of(commonRoom), List.of());
    }
}
