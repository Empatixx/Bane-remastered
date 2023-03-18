package cz.empatix.gamestate.domain;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.gamestate.service.GameStateService;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.rooms.RoomMap;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

public class InGameState extends GameState {
    private final static Logger LOGGER = LoggerFactory.getLogger(InGameState.class);
    private RoomMap roomMap;

    public InGameState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.INGAME);
    }

    @Override
    public void init() {
        roomMap = new RoomMap();
    }

    @Override
    public void update(final double dt) {
        final Vector2f mouseDirection = new Vector2f(WindowHandler.TARGET_WIDTH / 2 - mouseX, WindowHandler.TARGET_HEIGHT / 2 - mouseY);
        Camera.setPosition(mouseDirection.x / 1500, mouseDirection.y / 1500);
    }

    @Override
    public void fixedUpdate(double dt) {
    }

    @Override
    public List<Drawable> toRender(final double dt) {
        return roomMap.toRender(dt);
    }

    @Override
    public void cleanUp() {

    }

    @Override
    public void keyPressed(int key) {

    }

    @Override
    public void keyReleased(int key) {

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
