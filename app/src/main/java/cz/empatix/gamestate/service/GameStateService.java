package cz.empatix.gamestate.service;

import cz.empatix.audio.service.AudioService;
import cz.empatix.collision.DamageHandler;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.newphysics.service.PhysicsDebugRenderer;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.postprocess.RedVignette;
import cz.empatix.postprocessing.domain.Fade;
import cz.empatix.postprocessing.domain.GaussianBlur;
import cz.empatix.render.service.Camera;
import cz.empatix.render.service.RenderEngine;
import cz.empatix.settings.domain.GameSettings;
import cz.empatix.settings.service.SettingsService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;

public class GameStateService {
    private final static Logger LOGGER = LoggerFactory.getLogger(GameStateService.class);
    private final static boolean DEBUG_PHYSICS = false;
    private final PhysicsDebugRenderer physicsDebugRenderer = new GlPhysicsDebugRenderer();
    private final List<GameState> gameStates;
    private final PhysicsService physicsService;
    private final RenderEngine renderEngine;
    private final InteractionService interactionService;
    private final AudioService audioService;
    private GameState currentState;

    public GameStateService() {
        physicsService = new PhysicsService();
        physicsService.addCollisionHandler(new DamageHandler());

        this.gameStates = new ArrayList<>();
        gameStates.add(new LoadingState(this));
        gameStates.add(new InGameState(this, physicsService));
        gameStates.add(new MenuState(this));
        gameStates.add(new LobbyState(this, physicsService));
        interactionService = InteractionService.getInstance();
        renderEngine = new RenderEngine(List.of(
                new GaussianBlur(),
                new Fade(),
                new RedVignette()
        ));
        audioService = AudioService.getInstance();
        final GameSettings settings = SettingsService.getInstance().getSettings();
        audioService.setVolume(settings.getOverallVolume(), settings.getMusicVolume(), settings.getEffectsVolume());

        GameStateController.init(this::setCurrentState);

        setCurrentState(GameStateType.LOADING);
    }
    public void setCurrentState(final GameStateType type){
        if(currentState != null) {
            currentState.cleanUp();
        }
        currentState = gameStates.stream()
                .filter(gameState -> gameState.getType() == type)
                .findFirst()
                .orElseThrow(() -> new IllegalArgumentException("No such state"));
        Camera.getInstance().setPosition(0,0);
        currentState.init();
        LOGGER.info("Current state set to {}", type);
    }
    public void update(final double dt) {
        currentState.updateMouse((float)dt);
        currentState.update(dt);
        audioService.update((float)dt);
        physicsService.update(dt);
        LightningService.getInstance().update();
    }
    public void fixedUpdate(final double dt) {
        interactionService.fixedUpdate((float)dt);
        currentState.fixedUpdate(dt);
        physicsService.fixedUpdate(dt);
    }
    public void render(final double dt) {
        Camera.getInstance().update((float) dt);
        renderEngine.render(currentState.toRender(dt), dt);
        if (DEBUG_PHYSICS) {
            physicsService.debugRender(physicsDebugRenderer);
        }
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
            //Camera.zoomIn((float)yoffset/100f);
        } else if(yoffset < 0){
            //Camera.zoomOut((float)-yoffset/100f);
        }
    }
}
