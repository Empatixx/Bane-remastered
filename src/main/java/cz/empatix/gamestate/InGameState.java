package cz.empatix.gamestate;

import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.rooms.RoomMap;

import java.util.List;

public class InGameState extends GameState{
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
        Camera.setPosition(mouseX/3000,mouseY/3000);
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
