package cz.empatix.gamestate;

import cz.empatix.assets.service.AssetsManager;
import org.lwjgl.opengl.GL30;

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
        GL30.glBegin(GL30.GL_QUADS);
        GL30.glColor3f(1,0,0);
        GL30.glVertex2f(-0.5f,0.5f);
        GL30.glVertex2f(0.5f,0.5f);
        GL30.glVertex2f(0.5f,-0.5f);
        GL30.glVertex2f(-0.5f,-0.5f);
        GL30.glEnd();

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
