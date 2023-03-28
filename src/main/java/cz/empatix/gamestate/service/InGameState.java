package cz.empatix.gamestate.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.player.Player;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.rooms.RoomMap;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.stream.Stream;

public class InGameState extends GameState {
    private final static Logger LOGGER = LoggerFactory.getLogger(InGameState.class);
    private RoomMap roomMap;
    private Player player;

    public InGameState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.INGAME);
    }

    @Override
    public void init() {
        roomMap = new RoomMap();
        player = new Player();
    }

    @Override
    public void update(final double dt) {
        final Vector2f mouseDirection = new Vector2f(WindowHandler.TARGET_WIDTH / 2f - mouseX, WindowHandler.TARGET_HEIGHT / 2f - mouseY);
        player.update(dt);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);

        final Vector2f mouseDirection = new Vector2f((mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f, (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f);
        Camera.setSmoothlyPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH/2f + mouseDirection.x(),player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT/2f + mouseDirection.y());
    }

    @Override
    public List<Drawable> toRender(final double dt) {
        return Stream.concat(
                roomMap.toRender(dt).stream(),
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
