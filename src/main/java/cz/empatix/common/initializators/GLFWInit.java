package cz.empatix.common.initializators;
import cz.empatix.common.handlers.WindowHandler;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.glfw.GLFWErrorCallback;
import org.lwjgl.glfw.GLFWImage;
import org.lwjgl.glfw.GLFWVidMode;
import org.lwjgl.system.MemoryStack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.ByteBuffer;
import java.nio.IntBuffer;

import static org.lwjgl.glfw.Callbacks.glfwFreeCallbacks;
import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.stb.STBImage.stbi_image_free;
import static org.lwjgl.stb.STBImage.stbi_load;
import static org.lwjgl.system.MemoryStack.stackPush;
import static org.lwjgl.system.MemoryUtil.NULL;

public class GLFWInit implements Initializator {
    private final static Logger LOGGER = LoggerFactory.getLogger(GLFWInit.class);
    private static final int HEIGHT = WindowHandler.TARGET_HEIGHT * 2;
    private static final int WIDTH = WindowHandler.TARGET_WIDTH * 2;
    private static final boolean VSYNC = true;
    public static long window;

    @Override
    public void init() {
        if (!GLFW.glfwInit()) {
            LOGGER.error("Unable to initialize GLFW");
            throw new IllegalStateException("Unable to initialize GLFW");
        }
        GLFWErrorCallback.createPrint(System.err).set();
        initWindow();
    }

    private void initWindow() {
        glfwDefaultWindowHints(); // optional, the current window hints are already the default
        glfwWindowHint(GLFW_VISIBLE, GLFW_FALSE); // the window will stay hidden after creation
        glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE); // the window will be resizable
        glfwWindowHint(GLFW_OPENGL_DEBUG_CONTEXT, GLFW_TRUE);

        // Create the window
        window = glfwCreateWindow(WIDTH, HEIGHT, "Bane", NULL, NULL);
        if (window == NULL) {
            throw new RuntimeException("Failed to create the GLFW window");
        }
        glfwSetWindowAspectRatio(window, WIDTH, HEIGHT);
        glfwSetWindowSizeLimits(window, WindowHandler.TARGET_WIDTH, WindowHandler.TARGET_HEIGHT, 2540, 1440);
        centerWindow();
        setIcon();
        // Make the OpenGL context current
        glfwMakeContextCurrent(window);
        // Enable v-sync
        glfwSwapInterval(VSYNC ? 1 : 0);
        glfwShowWindow(window);

        // hide cursor
        glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_HIDDEN);
    }

    private void centerWindow() {
        // Get the thread stack and push a new frame
        try (final MemoryStack stack = stackPush()) {
            final IntBuffer pWidth = stack.mallocInt(1); // int*
            final IntBuffer pHeight = stack.mallocInt(1); // int*

            // Get the window size passed to glfwCreateWindow
            glfwGetWindowSize(window, pWidth, pHeight);

            // Get the resolution of the primary monitor
            GLFWVidMode vidmode = glfwGetVideoMode(glfwGetPrimaryMonitor());

            // Center the window
            glfwSetWindowPos(
                    window,
                    (vidmode.width() - pWidth.get(0)) / 2,
                    (vidmode.height() - pHeight.get(0)) / 2
            );
        } // the stack frame is popped automatically
    }

    private void setIcon() {
        try (final MemoryStack stack = stackPush()) {
            final IntBuffer width = stack.mallocInt(1);
            final IntBuffer height = stack.mallocInt(1);
            final IntBuffer channels = stack.mallocInt(1);
            final ByteBuffer image = stbi_load("./assets/textures/icon.png", width, height, channels, 4);
            if (image != null) {
                final GLFWImage.Buffer icons = GLFWImage.malloc(1);
                icons.position(0).width(width.get(0)).height(height.get(0)).pixels(image);
                glfwSetWindowIcon(window, icons);
                icons.free();
                stbi_image_free(image);
            }
        }
    }

    @Override
    public void destroy() {
        glfwFreeCallbacks(window);
        glfwDestroyWindow(window);
        GLFW.glfwTerminate();
        GLFW.glfwSetErrorCallback(null).free();
    }
}
