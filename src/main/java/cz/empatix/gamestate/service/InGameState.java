package cz.empatix.gamestate.service;

import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.enemies.service.EnemyService;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.guns.service.GunService;
import cz.empatix.map.generators.ProceduralMapGenerator;
import cz.empatix.map.service.MapService;
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
    private MapService mapService;
    private GunService gunService;
    private Player player;

    public InGameState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.INGAME);
    }

    @Override
    public void init() {
        final ProceduralMapGenerator proceduralMapGenerator = new ProceduralMapGenerator();

        mapService = proceduralMapGenerator.generate();
        player = new Player();
        gunService = new GunService();

        AudioService.getInstance().play(Soundtrack.INGAME);
    }

    @Override
    public void update(final double dt) {
        final float mouseDirectionX = (mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f;
        final float mouseDirectionY = (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f;
        Camera.setSmoothlyPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f + mouseDirectionX, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f + mouseDirectionY, (float) dt);

        player.update(dt);
        mapService.update(dt, player);
        gunService.update(dt);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);
        gunService.fixedUpdate(player.getX(), player.getY(), Camera.getX() + mouseX, Camera.getY() + mouseY, dt);
    }

    @Override
    public List<Renderable> toRender(final double dt) {
        return Stream.of(mapService.toRender(dt), List.of(player), gunService.toRender(dt))
                .flatMap(List::stream)
                .collect(Collectors.toList());
    }

    @Override
    public void cleanUp() {

    }

    @Override
    public void keyPressed(int key) {
        player.keyPressed(key);
    }

    @Override
    public void keyReleased(int key) {
        player.keyReleased(key);
        player.setHealth(player.getHealth() + 1);

    }

    @Override
    public void mousePressed(int button) {
        gunService.setShooting(true);
        player.setHealth(player.getHealth() - 1);
    }

    @Override
    public void mouseReleased(int button) {
        gunService.setShooting(false);
    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }
}
