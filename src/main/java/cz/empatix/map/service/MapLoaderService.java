package cz.empatix.map.service;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.rooms.service.Room;

import java.util.function.Consumer;

public class MapLoaderService {
    private MapService mapService;
    private MinimapService minimapService;
    private final Consumer<Room> minimapConsumer;
    private boolean loaded = false;
    public MapLoaderService(final Consumer<Room> minimapConsumer){
        this.minimapConsumer = minimapConsumer;
    }
    public void load(final MapGenerationResult generation){
        mapService = new MapService(generation, minimapConsumer);
        minimapService = new MinimapService(generation);
        loaded = true;
    }

    public MapService getMapService() {
        return mapService;
    }

    public MinimapService getMinimapService() {
        return minimapService;
    }
    public boolean isLoadedNewMap(){
        return loaded;
    }
    public void setLoadedUse(){
        loaded = false;
    }
}
