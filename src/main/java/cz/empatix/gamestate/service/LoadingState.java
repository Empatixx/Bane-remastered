package cz.empatix.gamestate.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.hud.service.image.AnimatedImage;
import cz.empatix.hud.service.CommonBackground;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.domain.RenderableState;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class LoadingState extends GameState {
    private static final Logger LOGGER = LoggerFactory.getLogger(LoadingState.class);
    private HUDElement loadingBg;
    private HUDElement loadingCircle;
    private long startTime;
    public LoadingState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.LOADING);
    }

    @Override
    public void init() {
        AssetsService.loadAssets();
        loadingBg = new CommonBackground("loading/loading.tga");
        loadingCircle = new AnimatedImage("loading/loading_image.tga", new Vector2f(WindowHandler.TARGET_WIDTH/2,380), 8, true);
        startTime = System.currentTimeMillis();
    }

    @Override
    public void update(final double dt) {
        loadingBg.update(dt);
        loadingCircle.update(dt);
        AssetsService.createAssets();
        if(AssetsService.isLoaded()){
            LOGGER.info("Loading took {} ms", System.currentTimeMillis() - startTime);
            setCurrentState(GameStateType.MENU);
        }
    }

    @Override
    public void fixedUpdate(double dt) {
    }

    @Override
    public void toRender(double dt, RenderableState state) {
        state.add(loadingBg).add(loadingCircle);
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
