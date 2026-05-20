package cz.empatix.core;

/**
 * Viewport / window dimensions shared between the windowing layer (which writes them) and
 * the rendering layer (which reads them). This is the single source of truth for sizes,
 * keeping the render engine free of any direct window-management dependency.
 */
public final class Viewport {

    /** Fixed internal render resolution the game is designed around. */
    public static final int TARGET_WIDTH = 960;
    public static final int TARGET_HEIGHT = 540;

    /** Current window framebuffer size, updated by the windowing layer on resize. */
    public static int WIDTH;
    public static int HEIGHT;

    /** Primary monitor height, set once by the windowing layer at startup. */
    public static int MAX_HEIGHT;

    private Viewport() {
    }
}
