package cz.empatix.gamestate.service;

import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.generators.MapGenerator;
import cz.empatix.map.generators.StaticMapGenerator;
import cz.empatix.map.service.MapService;
import cz.empatix.physics.service.PhysicsEngine;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.service.Camera;

import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class LobbyState extends GameState {
    private MapService mapService;
    private Player player;

    protected LobbyState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.LOBBY);
    }

    @Override
    public void init() {
        CursorService.setCursor(CursorType.DEFAULT);

        final MapGenerator proceduralMapGenerator = new StaticMapGenerator();
        final MapGenerationResult mapGenerationResult = proceduralMapGenerator.generate();

        mapService = new MapService(mapGenerationResult, room -> {
        });
        player = new Player();

        AudioService.getInstance().play(Soundtrack.PROGRESS_ROOM);
        Camera.setPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f);
    }

    @Override
    public void update(double dt) {
        final float mouseDirectionX = (mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f;
        final float mouseDirectionY = (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f;
        Camera.setSmoothlyPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f + mouseDirectionX, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f + mouseDirectionY, (float) dt);

        player.update(dt);
        mapService.update(dt, player);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);
        mapService.fixedUpdate(dt);
    }

    @Override
    public List<Renderable> toRender(double dt) {
        final List<Renderable> renderables = Stream.of(mapService.toRender(dt), List.of(player))
                .flatMap(List::stream)
                .collect(Collectors.toList());
        renderables.add(CursorService.getCursor());
        return List.copyOf(renderables);
    }

    @Override
    public void cleanUp() {
        player.cleanUp();
        PhysicsEngine.clear();
        LightningService.getInstance().clear();
    }

    @Override
    public void keyPressed(int key) {
        player.keyPressed(key);
    }

    @Override
    public void keyReleased(int key) {
        player.keyReleased(key);
    }

    @Override
    public void mousePressed(int button) {

    }

    @Override
    public void mouseReleased(int button) {

    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }
}
