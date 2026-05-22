package cz.empatix.core;

import java.nio.file.Path;

/**
 * Central location of the on-disk asset directories. The game is launched from the
 * repository root, so these are resolved relative to the working directory.
 */
public final class AssetPaths {

    /** Root directory holding all asset subfolders. */
    public static final Path BASE = Path.of("./assets/");

    public static final Path TEXTURES = BASE.resolve("textures");
    public static final Path AUDIO = BASE.resolve("audio");
    public static final Path SHADERS = BASE.resolve("shaders");
    public static final Path FONTS = BASE.resolve("fonts");

    private AssetPaths() {
    }
}
