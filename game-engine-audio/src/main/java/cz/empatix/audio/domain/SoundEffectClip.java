package cz.empatix.audio.domain;

/**
 * A short, fire-and-forget sound the engine can play. The concrete catalogue of
 * effects is defined by the game; the engine only needs a name (for logging) and
 * the asset path to look up the buffer.
 */
public interface SoundEffectClip {
    String name();

    String getPath();
}
