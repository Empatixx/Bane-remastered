package cz.empatix.gamestate.service;

import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.death.service.DeathService;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.guns.service.GunService;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.generators.MapGenerator;
import cz.empatix.map.generators.ProceduralMapGenerator;
import cz.empatix.map.service.MapService;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.pause.service.PauseService;
import cz.empatix.physics.service.PhysicsEngine;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.service.Camera;
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
    private GunService gunService;
    private Player player;
    private DeathService deathService;

    public InGameState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.INGAME);
    }

    @Override
    public void init() {
        CursorService.setCursor(CursorType.CROSSHAIR);

        final MapGenerator proceduralMapGenerator = new ProceduralMapGenerator();
        final MapGenerationResult mapGenerationResult = proceduralMapGenerator.generate();

        minimapService = new MinimapService(mapGenerationResult);
        mapService = new MapService(mapGenerationResult, room -> minimapService.visit(room));
        player = new Player();
        gunService = new GunService();
        pauseService = new PauseService();
        deathService = new DeathService();

        AudioService.getInstance().play(Soundtrack.INGAME);
        Camera.setPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f);
    }

    @Override
    public void update(final double dt) {
        deathService.update((float)dt, player.isDead());
        if(deathService.isActive()){
            player.update(dt);
            mapService.update(dt, player);
            return;
        }
        final float mouseDirectionX = (mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f;
        final float mouseDirectionY = (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f;
        Camera.setSmoothlyPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f + mouseDirectionX, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f + mouseDirectionY, (float) dt);
        player.update(dt);
        mapService.update(dt, player);
        minimapService.update(dt);
        gunService.update(dt);
        pauseService.update(dt, mouseX, mouseY);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);
        mapService.fixedUpdate(dt);
        gunService.fixedUpdate(player.getX() + player.getWidth()/2f, player.getY() + player.getHeight()/2f, Camera.getX() + mouseX, Camera.getY() + mouseY, dt);
    }

    @Override
    public List<Renderable> toRender(final double dt) {
        return Stream.of(mapService.toRender(dt), List.of(player), gunService.toRender(dt), pauseService.toRender(dt),
                        minimapService.toRender(dt), List.of(CursorService.getCursor()), deathService.toRender())
                .flatMap(List::stream)
                .collect(Collectors.toList());
    }

    @Override
    public void cleanUp() {
        player.cleanUp();
        PhysicsEngine.clear();
        LightningService.getInstance().clear();
    }

    @Override
    public void keyPressed(final int key) {
        pauseService.keyPressed(key);
        if(pauseService.isPause()){
            return;
        }
        if(deathService.isActive()){
            deathService.keyPressed(key);
            return;
        }
        gunService.keyPressed(key);
        player.keyPressed(key);
    }

    @Override
    public void keyReleased(final int key) {
        if(pauseService.isPause()){
            return;
        }
        if(deathService.isActive()){
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
