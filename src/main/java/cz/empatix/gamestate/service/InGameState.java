package cz.empatix.gamestate.service;

import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.death.service.DeathService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.guns.service.GunService;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.generators.ProceduralMapGenerator;
import cz.empatix.map.service.MapLoaderService;
import cz.empatix.map.service.MapService;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.pause.service.PauseService;
import cz.empatix.player.CoinService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.service.Room;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class InGameState extends GameState {
    private final static Logger LOGGER = LoggerFactory.getLogger(InGameState.class);
    private PauseService pauseService;
    private MinimapService minimapService;
    private MapService mapService;
    private MapLoaderService mapLoaderService;
    private GunService gunService;
    private ArtefactsService artefactsService;
    private CoinService coinService;
    private ItemService itemService;
    private Player player;
    private DeathService deathService;

    public InGameState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.INGAME);
    }

    @Override
    public void init() {
        CursorService.setCursor(CursorType.CROSSHAIR);

        final StatisticsService statisticsService = new StatisticsService();

        player = new Player();
        coinService = new CoinService();
        pauseService = new PauseService();
        deathService = new DeathService(statisticsService);
        itemService = new ItemService(statisticsService);
        gunService = new GunService(itemService, statisticsService);
        artefactsService = new ArtefactsService(player, itemService);

        itemService.setFactory(new ItemFactory(gunService, coinService, player, artefactsService));

        mapLoaderService = new MapLoaderService(itemService, room -> minimapService.visit(room), statisticsService);

        final ProceduralMapGenerator proceduralMapGenerator = new ProceduralMapGenerator();
        final MapGenerationResult mapGenerationResult = proceduralMapGenerator.generate(itemService, mapLoaderService, statisticsService);
        mapLoaderService.load(mapGenerationResult);
        mapLoaderService.setLoadedUse();

        mapService = mapLoaderService.getMapService();
        minimapService = mapLoaderService.getMinimapService();

        final Room startRoom = mapGenerationResult.startRoom();
        player.setPosition(startRoom.getX() + startRoom.getWidth() / 2f - player.getWidth()/2f,
                startRoom.getY() + startRoom.getHeight() / 2f - player.getHeight()/2f);

        AudioService.getInstance().play(Soundtrack.INGAME);
        Camera.setPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f);
    }

    @Override
    public void update(final double dt) {
        if (mapLoaderService.isLoadedNewMap()){
            mapLoaderService.setLoadedUse();
            mapService = mapLoaderService.getMapService();
            minimapService = mapLoaderService.getMinimapService();
            final Room startRoom = mapService.getCurrentRoom();
            player.setPosition(startRoom.getX() + startRoom.getWidth() / 2f - player.getWidth()/2f,
                    startRoom.getY() + startRoom.getHeight() / 2f - player.getHeight()/2f);
            Camera.setPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f);
        }
        deathService.update((float)dt, player.isDead());
        if(deathService.isActive()){
            gunService.setShooting(false);
            player.update(dt);
            mapService.update(dt, player);
            return;
        }
        final float mouseDirectionX = (mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f;
        final float mouseDirectionY = (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f;
        Camera.setSmoothlyPosition(player.getX() + player.getWidth()/2f - WindowHandler.TARGET_WIDTH / 2f + mouseDirectionX, player.getY() + player.getHeight()/2f - WindowHandler.TARGET_HEIGHT / 2f + mouseDirectionY, (float) dt);
        player.update(dt);
        mapLoaderService.update((float) dt);
        mapService.update(dt, player);
        minimapService.update(dt);
        gunService.update(dt);
        artefactsService.update(dt);
        itemService.update((float)dt);
        pauseService.update(dt, mouseX, mouseY);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);
        mapService.fixedUpdate(dt);
        gunService.fixedUpdate(player.getX() + player.getWidth()/2f, player.getY() + player.getHeight()/2f, Camera.getX() + mouseX, Camera.getY() + mouseY, dt);
        artefactsService.fixedUpdate(dt);
    }

    @Override
    public List<Renderable> toRender(final double dt) {
        return Stream.of(mapService.toRender(dt), List.of(player), gunService.toRender(dt), pauseService.toRender(dt),
                        minimapService.toRender(dt), List.of(CursorService.getCursor()), deathService.toRender(), itemService.toRender(),
                        mapLoaderService.toRender(), artefactsService.toRender(), coinService.toRender())
                .flatMap(List::stream)
                .collect(Collectors.toList());
    }

    @Override
    public void cleanUp() {
        player.cleanUp();
        PhysicsService.getInstance().clear();
        LightningService.getInstance().clear();
        InteractionService.getInstance().clear();
    }

    @Override
    public void keyPressed(final int key) {
        if(deathService.isActive()){
            deathService.keyPressed(key);
            return;
        }
        pauseService.keyPressed(key);
        if(pauseService.isPause()){
            return;
        }
        gunService.keyPressed(key);
        artefactsService.keyPressed(key);
        player.keyPressed(key);
    }

    @Override
    public void keyReleased(final int key) {
        if(deathService.isActive()){
            return;
        }
        if(pauseService.isPause()){
            return;
        }
        player.keyReleased(key);
    }

    @Override
    public void mousePressed(final int button) {
        if(pauseService.isPause()){
            pauseService.mousePressed(button, mouseX, mouseY);
            return;
        }
        if(deathService.isActive()){
            return;
        }
        gunService.setShooting(true);
    }

    @Override
    public void mouseReleased(int button) {
        if(pauseService.isPause()){
            return;
        }
        if(deathService.isActive()){
            return;
        }
        gunService.setShooting(false);
    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }
}
