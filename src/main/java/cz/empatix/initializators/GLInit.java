package cz.empatix.initializators;

import org.lwjgl.opengl.GL;
import org.lwjgl.opengl.GLDebugMessageCallback;
import org.lwjgl.opengl.KHRDebug;
import org.lwjgl.system.Configuration;

import static org.lwjgl.opengl.GL43.*;
import static org.lwjgl.system.MemoryUtil.NULL;

public class GLInit implements Initializator {
    private final static boolean DEBUG_MODE = false;

    @Override
    public void init() {
        GL.createCapabilities();

        glEnable(GL_TEXTURE_2D);
        glEnable(GL_BLEND);
        glEnable(GL_LINE_SMOOTH);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        glHint(GL_LINE_SMOOTH_HINT, GL_NICEST);

        glOrtho(0, 1920, 1080, 0, 1, -1);
        glViewport(0, 0, 1080, 1920); //TODO: make dynamic

        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        if (DEBUG_MODE) {
            Configuration.DEBUG_MEMORY_ALLOCATOR.set(true);
            enableDebug();
        }

    }

    private void enableDebug() {
        glEnable(GL_DEBUG_OUTPUT);
        glEnable(KHRDebug.GL_DEBUG_OUTPUT_SYNCHRONOUS);
        KHRDebug.glDebugMessageControl(GL_DONT_CARE, GL_DEBUG_TYPE_PERFORMANCE, GL_DEBUG_SEVERITY_HIGH, 0, true);
        KHRDebug.glDebugMessageCallback(new GLDebugMessageCallback() {
            @Override
            public void invoke(int source, int type, int id, int severity, int length, long message, long userParam) {
                if (type == GL_DEBUG_TYPE_PERFORMANCE) {
                    System.out.println(GLDebugMessageCallback.getMessage(length, message));

                }
            }
        }, NULL);
    }

    @Override
    public void destroy() {
        GL.destroy();
    }
}
