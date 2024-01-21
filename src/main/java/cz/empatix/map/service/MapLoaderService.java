package cz.empatix.map.service;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.function.Consumer;

public class MapLoaderService {
    private final static Logger LOGGER = LoggerFactory.getLogger(MapLoaderService.class);
    private final ItemService itemService;
    private final Consumer<Room> minimapConsumer;
    private boolean loaded = false;
    private MapService mapService;
    private MinimapService minimapService;
    private final StatisticsService statisticsService;
    private final MapTransitionService mapTransitionService;


    public MapLoaderService(final ItemService itemService,
                            final Consumer<Room> minimapConsumer,
                            final StatisticsService statisticsService) {
        this.minimapConsumer = minimapConsumer;
        this.itemService = itemService;
        this.statisticsService = statisticsService;
        this.mapTransitionService = new MapTransitionService();
    }
    public void load(final MapGenerationResult generation){
        if (mapService != null){
            LOGGER.info("Cleaning up old map");
            mapService.cleanUp();
            itemService.cleanUp();
            mapTransitionService.nextFloor();
        }
        LOGGER.info("Loading new map");
        mapService = new MapService(generation, minimapConsumer);
        minimapService = new MinimapService(generation);
        loaded = true;
        statisticsService.nextFloor();
    }

    public List<Renderable> toRender() {
        return mapTransitionService.toRender();
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
