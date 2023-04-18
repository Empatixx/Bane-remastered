package cz.empatix.gamestate.service;

import cz.empatix.gamestate.domain.*;
import cz.empatix.physics.service.PhysicsEngine;
import cz.empatix.postprocessing.service.PostprocessService;
import cz.empatix.render.Camera;
import cz.empatix.render.renders.RenderService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;

public class GameStateService {
    private final static Logger LOGGER = LoggerFactory.getLogger(GameStateService.class);
    private final List<GameState> gameStates;
    private final RenderService rendererService;
    private final PhysicsEngine physicsEngine;
    private final PostprocessService postprocessService;
    private GameState currentState;

    public GameStateService() {
        this.gameStates = new ArrayList<>();
        gameStates.add(new LoadingState(this));
        gameStates.add(new InGameState(this));
        gameStates.add(new MenuState(this));
        setCurrentState(GameStateType.LOADING);

        physicsEngine = new PhysicsEngine();
        rendererService = new RenderService();
        postprocessService = new PostprocessService();
    }
    public void setCurrentState(final GameStateType type){
        currentState = gameStates.stream()
                .filter(gameState -> gameState.getType() == type)
                .findFirst()
                .orElseThrow(() -> new IllegalArgumentException("No such state"));
        currentState.init();
        Camera.setPosition(0,0);
        LOGGER.info("Current state set to {}", type);
    }
    public void update(final double dt) {
        currentState.update(dt);
        physicsEngine.update(dt);
    }
    public void fixedUpdate(final double dt) {
        currentState.updateMouse();
        currentState.fixedUpdate(dt);
        physicsEngine.fixedUpdate(dt);
    }
    public void render(final double dt) {
        Camera.update((float) dt);
        rendererService.render(currentState.toRender(dt));
        postprocessService.renderPostprocessing(dt);
        physicsEngine.debugRender();
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
        if(yoffset > 0){
            Camera.zoomIn((float)yoffset/100f);
        } else if(yoffset < 0){
            Camera.zoomOut((float)-yoffset/100f);
        }
    }
}
