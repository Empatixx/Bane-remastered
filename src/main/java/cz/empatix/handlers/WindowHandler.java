package cz.empatix.handlers;

import cz.empatix.initializators.GLFWInit;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.opengl.GL30;
import org.lwjgl.system.MemoryStack;

import java.nio.IntBuffer;

import static org.lwjgl.system.MemoryStack.stackPush;

public class WindowHandler {
    public final static long WINDOW = GLFWInit.window;
    public static int WIDTH;
    public static int HEIGHT;

    public WindowHandler(){
        try (final MemoryStack stack = stackPush()){
            final IntBuffer pWidth = stack.mallocInt(1);
            final IntBuffer pHeight = stack.mallocInt(1);

            GLFW.glfwGetWindowSize(WINDOW, pWidth, pHeight);

            WIDTH = pWidth.get(0);
            HEIGHT = pHeight.get(0);
        }
        addResizeCallback();
        GL30.glViewport(0, 0, WIDTH, HEIGHT);
    }
    @SuppressWarnings("resource")
    private void addResizeCallback(){
        GLFW.glfwSetWindowSizeCallback(WINDOW, (window, width, height) -> {
            WIDTH = width;
            HEIGHT = height;
            GL30.glViewport(0, 0, width, height);
        });
    }
    public static float getAspectRatio(){
        return (float)WIDTH / (float)HEIGHT;
    }
}
