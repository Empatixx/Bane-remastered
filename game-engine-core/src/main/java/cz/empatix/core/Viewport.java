package cz.empatix.core;

/**
 * Viewport / window dimensions shared between the windowing layer (which writes them) and
 * the rendering layer (which reads them). This is the single source of truth for sizes,
 * keeping the render engine free of any direct window-management dependency.
 */
public final class Viewport {

    /**
     * Fixed internal render resolution. The concrete values are the application's
     * choice; the engine only stores them. Set once at startup via {@link #configure}.
     */
    public static int TARGET_WIDTH;
    public static int TARGET_HEIGHT;

    /** Current window framebuffer size, updated by the windowing layer on resize. */
    public static int WIDTH;
    public static int HEIGHT;

    /** Primary monitor height, set once by the windowing layer at startup. */
    public static int MAX_HEIGHT;

    private Viewport() {
    }

    /** Set the internal render resolution. Must be called by the application before rendering. */
    public static void configure(final int targetWidth, final int targetHeight) {
        TARGET_WIDTH = targetWidth;
        TARGET_HEIGHT = targetHeight;
    }
}
