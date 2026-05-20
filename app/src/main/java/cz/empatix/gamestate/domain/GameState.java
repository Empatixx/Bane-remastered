package cz.empatix.gamestate.domain;
import cz.empatix.core.Viewport;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.gamestate.service.GameStateService;
import cz.empatix.render.attributes.Renderable;
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
    public void updateMouse(final float dt){
        try (final MemoryStack stack = stackPush()) {
            final DoubleBuffer mouseX = stack.mallocDouble(1);
            final DoubleBuffer mouseY = stack.mallocDouble(1);
                GLFW.glfwGetCursorPos(WindowHandler.WINDOW, mouseX, mouseY);
            if (mouseX.get(0) < 0 || mouseX.get(0) > Viewport.WIDTH || mouseY.get(0) < 0 || mouseY.get(0) > Viewport.HEIGHT) {
                return;
            }
            this.mouseX = (float) mouseX.get(0) * Viewport.TARGET_WIDTH / Viewport.WIDTH;
            this.mouseY = (float) mouseY.get(0) * Viewport.TARGET_HEIGHT / Viewport.HEIGHT;
            if(AssetsService.getLoaded()){
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
    public abstract List<Renderable> toRender(final double dt);
    public abstract void cleanUp();
    public abstract void keyPressed(int key);
    public abstract void keyReleased(int key);
    public abstract void mousePressed(int button);
    public abstract void mouseReleased(int button);
    public abstract void mouseScrolled(double xoffset, double yoffset);
}
