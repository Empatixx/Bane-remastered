package cz.empatix.common.initializators;

import cz.empatix.core.GraphicsCapabilities;
import cz.empatix.settings.domain.GameSettings;
import cz.empatix.settings.service.SettingsService;
import org.lwjgl.opengl.GL;
import org.lwjgl.opengl.GLDebugMessageCallback;
import org.lwjgl.opengl.KHRDebug;
import org.lwjgl.system.Configuration;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import static org.lwjgl.opengl.GL43.*;
import static org.lwjgl.system.MemoryUtil.NULL;

public class GLInit implements Initializer {
    private final static Logger LOGGER = LoggerFactory.getLogger(GLInit.class);
    private final static boolean DEBUG_MODE = true;

    @Override
    public void init() {
        GL.createCapabilities();

        glEnable(GL_TEXTURE_2D);

        glEnable(GL_BLEND);
        glBlendFuncSeparate(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA, GL_ONE, GL_ONE_MINUS_SRC_ALPHA);

        glEnable(GL_LINE_SMOOTH);
        glHint(GL_LINE_SMOOTH_HINT, GL_NICEST);

        glEnable(GL_DEPTH_TEST);

        final GameSettings settings = SettingsService.getInstance().getSettings();
        glViewport(0, 0, settings.getResolutionWidth(), settings.getResolutionHeight());

        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        if (DEBUG_MODE) {
            LOGGER.info("Debug mode enabled");
            enableDebug();
        }

        GraphicsCapabilities.maxTextureSize = glGetInteger(GL_MAX_TEXTURE_SIZE);

    }

    private void enableDebug() {
        glEnable(GL_DEBUG_OUTPUT);
        glEnable(GL_DEBUG_OUTPUT_SYNCHRONOUS);
        Configuration.DEBUG_MEMORY_ALLOCATOR.set(true);
        KHRDebug.glDebugMessageCallback(new GLDebugMessageCallback() {
            @Override
            public void invoke(int source, int type, int id, int severity, int length, long message, long userParam) {
                if (type == GL_DEBUG_TYPE_ERROR) {
                    LOGGER.error("GL error: {}", GLDebugMessageCallback.getMessage(length, message));
                } else if (type == GL_DEBUG_TYPE_PERFORMANCE){
                    LOGGER.warn("GL performance warning: {}", GLDebugMessageCallback.getMessage(length, message));
                } else {
                    LOGGER.info("GL info: {}", GLDebugMessageCallback.getMessage(length, message));
                }
            }
        }, NULL);
    }

    @Override
    public void destroy() {
        GL.destroy();
    }
}
