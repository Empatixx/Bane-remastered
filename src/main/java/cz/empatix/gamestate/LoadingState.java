package cz.empatix.gamestate;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Background;
import org.lwjgl.opengl.GL30;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class LoadingState extends GameState{
    private static final Logger LOGGER = LoggerFactory.getLogger(LoadingState.class);
    private Background loadingBg;
    protected LoadingState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.LOADING);
    }

    @Override
    void init() {
        AssetsService.loadAssets();
        loadingBg = new Background("Menu/loading.tga");
    }

    @Override
    void update() {
        AssetsService.createAssets();
        if(AssetsService.isLoaded()){
            setCurrentState(GameStateType.INGAME);
            LOGGER.info("Assets loaded");
        }
    }

    @Override
    void render() {
        GL30.glBegin(GL30.GL_QUADS);
        GL30.glColor3f(1,0,0);
        GL30.glVertex2f(0,150);
        GL30.glVertex2f(150,150);
        GL30.glVertex2f(150,0);
        GL30.glVertex2f(0,0);
        GL30.glEnd();

        loadingBg.draw();
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
