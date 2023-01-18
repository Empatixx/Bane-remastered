package cz.empatix.gamestate;

import cz.empatix.handlers.WindowHandler;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.system.MemoryStack;

import java.nio.DoubleBuffer;

import static org.lwjgl.system.MemoryStack.stackPush;

public abstract class GameState {
    private final GameStateService gameStateService;
    private final GameStateType type;
    private float mouseX, mouseY;

    protected GameState(final GameStateService gameStateService,
                        final GameStateType type) {
        this.gameStateService = gameStateService;
        this.type = type;
    }
    private void updateMouse(){
        try (MemoryStack stack = stackPush()) {
            final DoubleBuffer mouseX = stack.mallocDouble(1);
            final DoubleBuffer mouseY = stack.mallocDouble(1);
            GLFW.glfwGetCursorPos(WindowHandler.WINDOW, mouseX, mouseY);
            this.mouseX = (float) mouseX.get(0);
            this.mouseY = (float) mouseY.get(0);
        }
    }
    public void setCurrentState(final GameStateType type){
        gameStateService.setCurrentState(type);
    }
    public GameStateType getType(){
        return type;
    }
    abstract void init();
    abstract void update();
    abstract void render();
    abstract void cleanUp();
    abstract void keyPressed(int key);
    abstract void keyReleased(int key);
    abstract void mousePressed(int button);
    abstract void mouseReleased(int button);
    abstract void mouseScrolled(double xoffset, double yoffset);
}
