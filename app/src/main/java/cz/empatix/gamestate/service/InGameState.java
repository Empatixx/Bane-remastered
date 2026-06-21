package cz.empatix.gamestate.service;
import cz.empatix.core.Viewport;

import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.sound.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.RngService;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.death.service.DeathService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.enemies.service.DifficultyService;
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
import cz.empatix.pets.PetService;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.player.CoinService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.service.Room;
import cz.empatix.shadow.domain.ShadowCaster;
import cz.empatix.shadow.service.ObjectShadowService;
import cz.empatix.shadow.service.ShadowService;
import cz.empatix.upgrades.appliers.UpgradesApplicationService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;

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
    private PetService petService;
    private ShadowService shadowService;
    private ObjectShadowService objectShadowService;
    private final PhysicsService physicsService;

    public InGameState(final GameStateService gameStateService, final PhysicsService physicsService) {
        super(gameStateService, GameStateType.INGAME);
        this.physicsService = physicsService;
    }

    @Override
    public void init() {
        CursorService.setCursor(CursorType.CROSSHAIR);

        final StatisticsService statisticsService = new StatisticsService();
        final ActiveUpgradeService activeUpgradeService = new ActiveUpgradeService();

        player = new Player();
        coinService = new CoinService();
        pauseService = new PauseService();
        deathService = new DeathService(statisticsService);
        itemService = new ItemService(statisticsService);

        minimapService = new MinimapService();

        mapService = new MapService();
        mapService.onVisit(minimapService::visit);

        gunService = new GunService(itemService, statisticsService);
        artefactsService = new ArtefactsService(player, itemService, minimapService);


        itemService.setFactory(new ItemFactory(gunService, coinService, player, artefactsService));

        mapLoaderService = new MapLoaderService(itemService, mapService, minimapService, statisticsService);
        mapLoaderService.addListener(result -> {
            final Room startRoom = result.startRoom();
            player.setPosition(startRoom.getX() + startRoom.getWidth() / 2f - player.getWidth()/2f,
                    startRoom.getY() + startRoom.getHeight() / 2f - player.getHeight()/2f);
            Camera.getInstance().setPosition(player.getX() + player.getWidth() - Viewport.TARGET_WIDTH / 2f, player.getY() + player.getHeight() - Viewport.TARGET_HEIGHT / 2f);
        });


        final DifficultyService difficultyService = new DifficultyService();
        final RngService rngService = new RngService(System.currentTimeMillis());
        final ProceduralMapGenerator proceduralMapGenerator = new ProceduralMapGenerator();
        final MapGenerationResult mapGenerationResult = proceduralMapGenerator.generate(itemService, mapLoaderService, statisticsService, coinService, difficultyService, rngService);
        mapLoaderService.load(mapGenerationResult);

        petService = new PetService(player);
        shadowService = new ShadowService(player);
        objectShadowService = new ObjectShadowService();

        final UpgradesApplicationService upgradesApplicationService = new UpgradesApplicationService(activeUpgradeService, player, gunService, mapService);
        upgradesApplicationService.apply();

        AudioService.getInstance().play(Soundtrack.INGAME);
        Camera.getInstance().setPosition(player.getX() + player.getWidth() - Viewport.TARGET_WIDTH / 2f, player.getY() + player.getHeight() - Viewport.TARGET_HEIGHT / 2f);
    }

    @Override
    public void update(final double dt) {
        deathService.update((float)dt, player.isDead());
        if(deathService.isActive()){
            gunService.setShooting(false);
            pauseService.setPause(false);
            player.update(dt);
            mapService.update(dt, player);
            return;
        }
        if (!minimapService.isActiveBigMinimap()){
            final float mouseDirectionX = (mouseX - Viewport.TARGET_WIDTH / 2f) / 10f;
            final float mouseDirectionY = (mouseY - Viewport.TARGET_HEIGHT / 2f) / 10f;
            Camera.getInstance().setSmoothlyPosition(player.getX() + player.getWidth()/2f - Viewport.TARGET_WIDTH / 2f + mouseDirectionX, player.getY() + player.getHeight()/2f - Viewport.TARGET_HEIGHT / 2f + mouseDirectionY, (float) dt);
        } else {
            Camera.getInstance().setSmoothlyPosition(player.getX() + player.getWidth()/2f - Viewport.TARGET_WIDTH / 2f, player.getY() + player.getHeight()/2f - Viewport.TARGET_HEIGHT / 2f, (float) dt);
        }
        player.update(dt);
        shadowService.update();
        mapLoaderService.update((float) dt);
        mapService.update(dt, player);
        minimapService.update((float)dt, mouseX, mouseY);
        gunService.update(dt);
        artefactsService.update(dt);
        itemService.update((float)dt);
        pauseService.update(dt, mouseX, mouseY);
        petService.update((float)dt);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);
        mapService.fixedUpdate(dt);
        gunService.fixedUpdate(player.getX() + player.getWidth()/2f, player.getY() + player.getHeight()/2f, Camera.getInstance().getX() + mouseX, Camera.getInstance().getY() + mouseY, dt);
        artefactsService.fixedUpdate(dt);
        petService.fixedUpdate((float)dt);
    }

    @Override
    public List<Renderable> toRender(final double dt) {
        final RenderableState coinState = new RenderableState(8);
        coinService.collectSubRenderables(coinState);

        // Collect shadow casters for object shadows (enemies and pets only)
        final List<ShadowCaster> shadowCasters = new ArrayList<>();
        for (final Room room : mapService.getRooms()) {
            shadowCasters.addAll(room.getEnemies());
        }
        if (petService.getPet().canRender()) {
            shadowCasters.add(petService.getPet());
        }

        // Compute object shadows (each object gets its own shadow angle)
        final List<Renderable> objectShadows = objectShadowService.computeShadows(
                shadowCasters,
                (float) dt
        );

        final List<Renderable> renderables = new ArrayList<>();
        renderables.addAll(mapService.toRender(dt));
        renderables.addAll(objectShadows);
        renderables.addAll(shadowService.toRender((float) dt));
        renderables.add(player);
        renderables.addAll(gunService.toRender(dt));
        renderables.addAll(pauseService.toRender(dt));
        renderables.addAll(minimapService.toRender(dt));
        renderables.add(CursorService.getCursor());
        renderables.addAll(deathService.toRender());
        renderables.addAll(itemService.toRender());
        renderables.addAll(mapLoaderService.toRender());
        renderables.addAll(artefactsService.toRender());
        renderables.addAll(coinState.getList());
        renderables.addAll(petService.toRender());
        return renderables;
    }


    @Override
    public void cleanUp() {
        player.cleanUp();
        physicsService.clear();
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
            player.stopMoving();
            minimapService.deactivateBigMinimap();
            return;
        }
        minimapService.keyPressed(key);
        player.keyPressed(key);
        if (minimapService.isActiveBigMinimap()){
            gunService.setShooting(false);
            return;
        }
        gunService.keyPressed(key);
        artefactsService.keyPressed(key);
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
        minimapService.mousePressed(button, mouseX, mouseY);
        if (minimapService.isActiveBigMinimap()){
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
        minimapService.mouseReleased(button, mouseX, mouseY);
        if (minimapService.isActiveBigMinimap()){
            return;
        }
        gunService.setShooting(false);
    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {
        gunService.mouseScrolled(xoffset, yoffset);
    }
}
