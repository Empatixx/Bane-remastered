package cz.empatix.gamestate.service;

import cz.empatix.gamestate.domain.GameStateType;

import java.util.function.Consumer;

public class GameStateController {
    private final Consumer<GameStateType> stateConsumer;
    private static GameStateController instance;
    public static GameStateController getInstance() {
        return instance;
    }
    public static void init(final Consumer<GameStateType> stateConsumer) {
        GameStateController.instance = new GameStateController(stateConsumer);
    }
    private GameStateController(final Consumer<GameStateType> stateConsumer) {
        this.stateConsumer = stateConsumer;
    }
    public void setCurrentState(final GameStateType type) {
        stateConsumer.accept(type);
    }
}
