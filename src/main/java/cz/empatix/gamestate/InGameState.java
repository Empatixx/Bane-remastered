package cz.empatix.gamestate;

import cz.empatix.render.attributes.Drawable;

import java.util.List;

public class InGameState extends GameState{
    public InGameState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.INGAME);
    }

    @Override
    void init() {

    }

    @Override
    void update(final double dt) {

    }

    @Override
    List<Drawable> toRender(final double dt) {
        return List.of();
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
