package cz.empatix.common.handlers;

import cz.empatix.common.GameLoop;
import cz.empatix.common.initializators.GLFWInit;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.system.MemoryStack;

import java.nio.IntBuffer;

import static org.lwjgl.opengl.GL11.glViewport;
import static org.lwjgl.system.MemoryStack.stackPush;

public class WindowHandler {
    public final static long WINDOW = GLFWInit.window;
    public static int WIDTH;
    public static int HEIGHT;
    public final static int TARGET_WIDTH = 960;
    public final static int TARGET_HEIGHT = 540;

    public WindowHandler(final GameLoop gameLoop){
        try (final MemoryStack stack = stackPush()){
            final IntBuffer pWidth = stack.mallocInt(1);
            final IntBuffer pHeight = stack.mallocInt(1);

            GLFW.glfwGetWindowSize(WINDOW, pWidth, pHeight);

            WIDTH = pWidth.get(0);
            HEIGHT = pHeight.get(0);
        }
        addResizeCallback(gameLoop);
    }

    public static int getMaxHeight() {
        //GLFW get max width of monitor
        return GLFW.glfwGetVideoMode(GLFW.glfwGetPrimaryMonitor()).height();
    }

    @SuppressWarnings("resource")
    private void addResizeCallback(final GameLoop gameLoop){
        GLFW.glfwSetFramebufferSizeCallback(WINDOW, (window, width, height) -> {
            WIDTH = width;
            HEIGHT = height;
            glViewport(0, 0, width, height);

        });
        //todo: remove in new GLFW (fixed)
        GLFW.glfwSetWindowSizeCallback(WINDOW, (window, width, height) -> {
            gameLoop.render(0);
        });
    }
    public static float getAspectRatio(){
        return (float)WIDTH / (float)HEIGHT;
    }
    public static float getScaleX(){
        return (float)WIDTH / (float)TARGET_WIDTH;
    }
}
