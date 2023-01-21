package cz.empatix.gamestate;

import cz.empatix.handlers.WindowHandler;
import cz.empatix.render.Drawable;
import cz.empatix.render.Renderer;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.system.MemoryStack;

import java.nio.DoubleBuffer;
import java.util.List;

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
    protected void updateMouse(){
        try (MemoryStack stack = stackPush()) {
            final DoubleBuffer mouseX = stack.mallocDouble(1);
            final DoubleBuffer mouseY = stack.mallocDouble(1);
            GLFW.glfwGetCursorPos(WindowHandler.WINDOW, mouseX, mouseY);
            this.mouseX = (float) mouseX.get(0) * WindowHandler.TARGET_WIDTH / WindowHandler.WIDTH;
            this.mouseY = (float) mouseY.get(0) * WindowHandler.TARGET_HEIGHT / WindowHandler.HEIGHT;
        }
    }
    public void setCurrentState(final GameStateType type){
        gameStateService.setCurrentState(type);
    }
    public GameStateType getType(){
        return type;
    }
    abstract void init();
    abstract void update(final double dt);
    abstract List<Drawable> toRender(final double dt);
    abstract void cleanUp();
    abstract void keyPressed(int key);
    abstract void keyReleased(int key);
    abstract void mousePressed(int button);
    abstract void mouseReleased(int button);
    abstract void mouseScrolled(double xoffset, double yoffset);
}
