package cz.empatix.common.handlers;

import com.google.common.base.Preconditions;
import cz.empatix.common.GameLoop;
import cz.empatix.common.initializators.GLFWInit;
import cz.empatix.settings.domain.GameSettings;
import cz.empatix.settings.service.SettingsService;
import org.joml.Vector2i;
import org.lwjgl.glfw.GLFW;
import org.lwjgl.glfw.GLFWVidMode;
import org.lwjgl.system.MemoryStack;

import java.nio.IntBuffer;
import java.util.List;
import java.util.stream.Stream;

import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.glfw.GLFW.glfwSetWindowPos;
import static org.lwjgl.opengl.GL11.glViewport;
import static org.lwjgl.system.MemoryStack.stackPush;

public class WindowHandler {
    public static int WIDTH;
    public static int HEIGHT;
    public final static int TARGET_WIDTH = 960;
    public final static int TARGET_HEIGHT = 540;
    public static final long WINDOW = GLFWInit.window;

    public static void registerCallbacks(final GameLoop gameLoop) {
        try (final MemoryStack stack = stackPush()) {
            final IntBuffer pWidth = stack.mallocInt(1);
            final IntBuffer pHeight = stack.mallocInt(1);

            GLFW.glfwGetWindowSize(WINDOW, pWidth, pHeight);
            WIDTH = pWidth.get(0);
            HEIGHT = pHeight.get(0);
        }

        GLFW.glfwSetFramebufferSizeCallback(WINDOW, (win, width, height) -> {
            final GameSettings settings = SettingsService.getInstance().getSettings();
            settings.setResolutionWidth(width);
            settings.setResolutionHeight(height);

            WIDTH = width;
            HEIGHT = height;
            glViewport(0, 0, width, height);

        });
        //todo: remove in new GLFW (fixed)
        GLFW.glfwSetWindowSizeCallback(WINDOW, (win, width, height) -> {
            gameLoop.render(0);
        });
    }
    private static void centerWindow() {
        try (final MemoryStack stack = stackPush()) {
            final IntBuffer pWidth = stack.mallocInt(1); // int*
            final IntBuffer pHeight = stack.mallocInt(1); // int*
            glfwGetWindowSize(WINDOW, pWidth, pHeight);
            final GLFWVidMode vidmode = glfwGetVideoMode(glfwGetPrimaryMonitor());
            // Center the window
            glfwSetWindowPos(
                    WINDOW,
                    (vidmode.width() - pWidth.get(0)) / 2,
                    (vidmode.height() - pHeight.get(0)) / 2
            );
        }
    }


    public static void setVsync(final boolean vsync) {
        GLFW.glfwSwapInterval(vsync ? 1 : 0);
    }

    public static void setResolution(final int width, final int height) {
        GLFW.glfwSetWindowSize(WINDOW, width, height);
    }
    public static List<Vector2i> getMonitorResolutions() {
        final long monitor = GLFW.glfwGetPrimaryMonitor();
        GLFWVidMode.Buffer videoModes = GLFW.glfwGetVideoModes(monitor);
        if (videoModes == null) {
            throw new RuntimeException("Failed to get video modes");
        }
        // Print all possible resolutions
        return Stream.generate(videoModes::get)
                .limit(videoModes.capacity())
                .map(vidMode -> new Vector2i(vidMode.width(), vidMode.height()))
                .distinct()
                .toList();
    }

    public static Vector2i getMonitorResolution() {
        final long monitor = GLFW.glfwGetPrimaryMonitor();
        final GLFWVidMode glfwVidMode = GLFW.glfwGetVideoMode(monitor);
        Preconditions.checkNotNull(glfwVidMode);
        return new Vector2i(glfwVidMode.width(), glfwVidMode.height());
    }

    public static void setFullscreen(final boolean full) {
        if (full) {
            final Vector2i monitorResolution = getMonitorResolution();
            GLFW.glfwSetWindowMonitor(WINDOW, GLFW.glfwGetPrimaryMonitor(),
                    0, 0, monitorResolution.x, monitorResolution.y, GLFW.GLFW_DONT_CARE);
        } else {
            GLFW.glfwSetWindowMonitor(WINDOW, 0, 0, 0, TARGET_WIDTH, TARGET_HEIGHT, GLFW.GLFW_DONT_CARE);
        }
    }

    public static int getMaxHeight() {
        //GLFW get max width of monitor
        return getMonitorResolution().y;
    }

}
