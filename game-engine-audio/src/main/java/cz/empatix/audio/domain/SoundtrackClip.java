package cz.empatix.audio.domain;

/**
 * A looping background track the engine can play and cross-fade. The concrete
 * catalogue of tracks is defined by the game; the engine only needs a name (for
 * logging) and the asset path to look up the buffer.
 */
public interface SoundtrackClip {
    String name();

    String getPath();
}
