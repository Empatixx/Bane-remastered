package cz.empatix.gamestate;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.background.AnimatedBackground;
import cz.empatix.render.background.Background;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

public class LoadingState extends GameState{
    private static final Logger LOGGER = LoggerFactory.getLogger(LoadingState.class);
    private Background loadingBg;
    protected LoadingState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.LOADING);
    }

    @Override
    void init() {
        AssetsService.loadAssets();
        loadingBg = new AnimatedBackground("Menu/loading.tga","background-loading.json");
    }

    @Override
    void update(final double dt) {
        loadingBg.update();
        AssetsService.createAssets();
        if(AssetsService.isLoaded()){
            setCurrentState(GameStateType.MENU);
        }
    }

    @Override
    List<Drawable> toRender(final double dt) {
        return List.of(loadingBg);
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
