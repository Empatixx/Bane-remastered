package cz.empatix.gamestate;

import cz.empatix.assets.creators.AssetCreator;
import cz.empatix.assets.service.AssetsManager;

public class LoadingState extends GameState{
    protected LoadingState(final GameStateManager gameStateManager) {
        super(gameStateManager);
    }

    @Override
    void init() {
        AssetsManager.loadAssets();
    }

    @Override
    void update() {
        AssetsManager.createAssets();
    }

    @Override
    void render() {

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
