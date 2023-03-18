package cz.empatix.gamestate.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.gamestate.service.GameStateService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.background.AnimatedBackground;
import cz.empatix.render.background.Background;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

public class LoadingState extends GameState {
    private static final Logger LOGGER = LoggerFactory.getLogger(LoadingState.class);
    private Background loadingBg;
    public LoadingState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.LOADING);
    }

    @Override
    public void init() {
        AssetsService.loadAssets();
        loadingBg = new AnimatedBackground("Menu/loading.tga",8);
    }

    @Override
    public void update(final double dt) {
        loadingBg.update();
        AssetsService.createAssets();
        if(AssetsService.isLoaded()){
            setCurrentState(GameStateType.MENU);
        }
    }

    @Override
    public void fixedUpdate(double dt) {
    }

    @Override
    public List<Drawable> toRender(final double dt) {
        return List.of(loadingBg);
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
