package cz.empatix.gamestate.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.gamestate.service.GameStateService;
import cz.empatix.render.domain.RenderableState;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.system.MemoryStack;

import java.nio.DoubleBuffer;

import static org.lwjgl.system.MemoryStack.stackPush;

public abstract class GameState {
    private final GameStateService gameStateService;
    private final GameStateType type;
    protected float mouseX, mouseY;

    protected GameState(final GameStateService gameStateService,
                        final GameStateType type) {
        this.gameStateService = gameStateService;
        this.type = type;
    }
    public void updateMouse(final float dt){
        try (final MemoryStack stack = stackPush()) {
            final DoubleBuffer mouseX = stack.mallocDouble(1);
            final DoubleBuffer mouseY = stack.mallocDouble(1);
                GLFW.glfwGetCursorPos(WindowHandler.WINDOW, mouseX, mouseY);
            this.mouseX = (float) mouseX.get(0) * WindowHandler.TARGET_WIDTH / WindowHandler.WIDTH;
            this.mouseY = (float) mouseY.get(0) * WindowHandler.TARGET_HEIGHT / WindowHandler.HEIGHT;

            if(AssetsService.isLoaded()){
                CursorService.updateMouse(this.mouseX, this.mouseY, dt);
            }
        }
    }
    public void setCurrentState(final GameStateType type){
        gameStateService.setCurrentState(type);
    }
    public GameStateType getType(){
        return type;
    }
    public abstract void init();
    public abstract void update(final double dt);
    public abstract void fixedUpdate(double dt);
    public abstract void toRender(double dt, RenderableState state);
    public abstract void cleanUp();
    public abstract void keyPressed(int key);
    public abstract void keyReleased(int key);
    public abstract void mousePressed(int button);
    public abstract void mouseReleased(int button);
    public abstract void mouseScrolled(double xoffset, double yoffset);
}
