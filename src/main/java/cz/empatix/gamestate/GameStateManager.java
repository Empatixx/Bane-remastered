package cz.empatix.gamestate;

import java.util.ArrayList;
import java.util.List;

public class GameStateManager {
    private final List<GameState> gameStates;
    private GameState currentState;
    public GameStateManager() {
        this.gameStates = new ArrayList<>();
        gameStates.add(new LoadingState(this));
        setCurrentState(0);
    }
    //TODO: redo to enum
    public void setCurrentState(final int state){
        currentState = gameStates.get(state);
        currentState.init();

    }
    public void update() {
        currentState.update();
    }

    public void render() {
        currentState.render();
    }

    public void keyPressed(int key) {
        currentState.keyPressed(key);
    }

    public void keyReleased(int key) {
        currentState.keyReleased(key);
    }

    public void mousePressed(int button) {
        currentState.mousePressed(button);
    }

    public void mouseReleased(int button) {
        currentState.mouseReleased(button);
    }

    public void mouseScrolled(double xoffset, double yoffset) {
        currentState.mouseScrolled(xoffset, yoffset);
    }
}
