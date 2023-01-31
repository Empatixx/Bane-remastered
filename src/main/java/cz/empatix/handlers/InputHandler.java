package cz.empatix.handlers;

import cz.empatix.common.GameLoop;
import cz.empatix.initializators.GLFWInit;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.glfw.GLFWKeyCallback;
import org.lwjgl.glfw.GLFWMouseButtonCallback;
import org.lwjgl.glfw.GLFWScrollCallback;

import static org.lwjgl.glfw.GLFW.GLFW_PRESS;
import static org.lwjgl.glfw.GLFW.GLFW_RELEASE;


public class InputHandler{

    private GLFWKeyCallback keyCallback;
    private GLFWMouseButtonCallback mouseButtonCallback;
    private GLFWScrollCallback scrollCallback;
    public InputHandler(final GameLoop gameLoop){
        keyCallback = new GLFWKeyCallback() {
            @Override
            public void invoke(long window, int key, int scancode, int action, int mods) {
                switch (action){
                    case GLFW_PRESS:
                        gameLoop.keyPressed(key);
                        break;
                    case GLFW_RELEASE:
                        gameLoop.keyReleased(key);
                        break;
                }
            }
        };
        mouseButtonCallback = new GLFWMouseButtonCallback() {
            @Override
            public void invoke(long window, int button, int action, int mods) {
                switch (action){
                    case GLFW_PRESS:
                        gameLoop.mousePressed(button);
                        break;
                    case GLFW_RELEASE:
                        gameLoop.mouseReleased(button);
                        break;
                }
            }
        };
        scrollCallback = new GLFWScrollCallback() {
            @Override
            public void invoke(long window, double xoffset, double yoffset) {
                gameLoop.mouseScrolled(xoffset, yoffset);
            }
        };
        GLFW.glfwSetKeyCallback(GLFWInit.window, keyCallback);
        GLFW.glfwSetMouseButtonCallback(GLFWInit.window, mouseButtonCallback);
        GLFW.glfwSetScrollCallback(GLFWInit.window, scrollCallback);
    }
}
