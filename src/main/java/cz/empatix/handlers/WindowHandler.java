package cz.empatix.handlers;

import cz.empatix.common.GameLoop;
import cz.empatix.initializators.GLFWInit;
import org.joml.Matrix4f;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.glfw.GLFWFramebufferSizeCallback;
import org.lwjgl.opengl.GL30;
import org.lwjgl.system.MemoryStack;

import java.nio.IntBuffer;

import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.system.MemoryStack.stackPush;

public class WindowHandler {
    public final static long WINDOW = GLFWInit.window;
    public static int WIDTH;
    public static int HEIGHT;
    public final static int TARGET_WIDTH = 640;
    public final static int TARGET_HEIGHT = 360;

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
}
