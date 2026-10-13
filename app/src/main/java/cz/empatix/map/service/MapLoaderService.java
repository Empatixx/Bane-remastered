package cz.empatix.map.service;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.service.ItemService;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

public class MapLoaderService {
    private final static Logger LOGGER = LoggerFactory.getLogger(MapLoaderService.class);
    private final ItemService itemService;
    private final MapService mapService;
    private final MinimapService minimapService;
    private final StatisticsService statisticsService;
    private final MapTransitionService mapTransitionService;
    private final List<Consumer<MapGenerationResult>> listeners;

    public MapLoaderService(final ItemService itemService,
                            final MapService mapService,
                            final MinimapService minimapService,
                            final StatisticsService statisticsService) {
        this.itemService = itemService;
        this.mapService = mapService;
        this.minimapService = minimapService;
        this.statisticsService = statisticsService;
        this.mapTransitionService = new MapTransitionService();
        this.listeners = new ArrayList<>();
    }
    public void clearPreviousMap(){
        LOGGER.info("Cleaning up old map");
        mapService.cleanUp();
        itemService.cleanUp();
        mapTransitionService.nextFloor();
    }
    public void load(final MapGenerationResult generation){
        LOGGER.info("Loading new map");
        minimapService.apply(generation);
        mapService.apply(generation);
        statisticsService.nextFloor();
        listeners.forEach(listener -> listener.accept(generation));
    }

    public List<Renderable> toRender() {
        return mapTransitionService.toRender();
    }
    public void addListener(final Consumer<MapGenerationResult> listener) {
        listeners.add(listener);
    }

    public void update(final float dt) {
        mapTransitionService.update(dt);
    }


}
