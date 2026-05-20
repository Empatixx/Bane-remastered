package cz.empatix.core;

/**
 * GPU capabilities queried once at GL initialization and consumed by the rendering layer
 * (e.g. texture-atlas packing). Written by the GL init code, read by render modules.
 */
public final class GraphicsCapabilities {

    /** Maximum 2D texture size supported by the GPU (GL_MAX_TEXTURE_SIZE). */
    public static int maxTextureSize;

    private GraphicsCapabilities() {
    }
}
