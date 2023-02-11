package cz.empatix.gamestate;

import cz.empatix.common.handlers.WindowHandler;
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
    void init() {
        roomMap = new RoomMap();
    }

    @Override
    void update(final double dt) {
        final Vector2f mouseDirection = new Vector2f(WindowHandler.TARGET_WIDTH / 2 - mouseX, WindowHandler.TARGET_HEIGHT / 2 - mouseY);
        Camera.setPosition(mouseDirection.x / 1500, mouseDirection.y / 1500);
    }

    @Override
    List<Drawable> toRender(final double dt) {
        return roomMap.toRender(dt);
    }

    @Override
    void cleanUp() {

    }

    @Override
    void keyPressed(int key) {

    }

    @Override
    void keyReleased(int key) {

    }

    @Override
    void mousePressed(int button) {

    }

    @Override
    void mouseReleased(int button) {

    }

    @Override
    void mouseScrolled(double xoffset, double yoffset) {

    }
}
