package cz.empatix.gamestate;

import java.util.ArrayList;
import java.util.List;

public class GameStateService {
    private final List<GameState> gameStates;
    private GameState currentState;
    public GameStateService() {
        this.gameStates = new ArrayList<>();
        gameStates.add(new LoadingState(this));
        gameStates.add(new InGameState(this));
        setCurrentState(GameStateType.LOADING);
    }
    public void setCurrentState(final GameStateType type){
        currentState = gameStates.stream()
                .filter(gameState -> gameState.getType() == type)
                .findFirst()
                .orElseThrow(() -> new IllegalArgumentException("No such state"));
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
