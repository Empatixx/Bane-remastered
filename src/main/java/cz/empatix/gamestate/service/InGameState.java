package cz.empatix.gamestate.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.player.Player;
import cz.empatix.render.service.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.map.service.MapService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.stream.Stream;

public class    InGameState extends GameState {
    private final static Logger LOGGER = LoggerFactory.getLogger(InGameState.class);
    private MapService mapService;
    private Player player;

    public InGameState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.INGAME);
    }

    @Override
    public void init() {
        mapService = new MapService();
        player = new Player();

    }

    @Override
    public void update(final double dt) {
        player.update(dt);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);

        final float mouseDirectionX = (mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f;
        final float mouseDirectionY = (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f;
        Camera.setSmoothlyPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH/2f + mouseDirectionX,player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT/2f + mouseDirectionY);
    }

    @Override
    public List<Drawable> toRender(final double dt) {
        return Stream.concat(
                mapService.toRender(dt).stream(),
                Stream.of(player)
        ).toList();
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
