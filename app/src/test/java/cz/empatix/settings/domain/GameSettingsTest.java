package cz.empatix.settings.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

class GameSettingsTest {

    private static GameSettings valid() {
        return new GameSettings(1920, 1080, true, false, 0.5f, 0.6f, 0.7f);
    }

    @Test
    void gettersReturnConstructorValues() {
        final GameSettings s = valid();

        assertEquals(1920, s.getResolutionWidth());
        assertEquals(1080, s.getResolutionHeight());
        assertTrue(s.isFullscreen());
        assertFalse(s.isVsync());
        assertEquals(0.5f, s.getOverallVolume());
        assertEquals(0.6f, s.getMusicVolume());
        assertEquals(0.7f, s.getEffectsVolume());
    }

    @Test
    void settersMutateState() {
        final GameSettings s = valid();
        s.setResolutionWidth(800);
        s.setResolutionHeight(600);
        s.setFullscreen(false);
        s.setVsync(true);
        s.setOverallVolume(0.1f);
        s.setMusicVolume(0.2f);
        s.setEffectsVolume(0.3f);

        assertEquals(800, s.getResolutionWidth());
        assertEquals(600, s.getResolutionHeight());
        assertFalse(s.isFullscreen());
        assertTrue(s.isVsync());
        assertEquals(0.1f, s.getOverallVolume());
        assertEquals(0.2f, s.getMusicVolume());
        assertEquals(0.3f, s.getEffectsVolume());
    }

    @Test
    void copyConstructorProducesIndependentCopy() {
        final GameSettings original = valid();
        final GameSettings copy = new GameSettings(original);

        // copy starts equal
        assertEquals(original.getResolutionWidth(), copy.getResolutionWidth());
        assertEquals(original.getOverallVolume(), copy.getOverallVolume());

        // mutating the copy must not affect the original
        copy.setResolutionWidth(640);
        copy.setOverallVolume(0.0f);
        assertEquals(1920, original.getResolutionWidth());
        assertEquals(0.5f, original.getOverallVolume());
    }

    @Test
    void deepCopyOverwritesAllFieldsFromSource() {
        final GameSettings target = valid();
        final GameSettings source = new GameSettings(640, 480, false, true, 0.1f, 0.2f, 0.3f);

        target.deepCopy(source);

        assertEquals(640, target.getResolutionWidth());
        assertEquals(480, target.getResolutionHeight());
        assertFalse(target.isFullscreen());
        assertTrue(target.isVsync());
        assertEquals(0.1f, target.getOverallVolume());
        assertEquals(0.2f, target.getMusicVolume());
        assertEquals(0.3f, target.getEffectsVolume());

        // mutating target afterwards must not write back into source
        target.setResolutionWidth(123);
        assertEquals(640, source.getResolutionWidth());
    }

    @Test
    void rejectsNonPositiveResolution() {
        assertThrows(IllegalArgumentException.class,
                () -> new GameSettings(0, 1080, true, false, 0.5f, 0.5f, 0.5f));
        assertThrows(IllegalArgumentException.class,
                () -> new GameSettings(1920, -1, true, false, 0.5f, 0.5f, 0.5f));
    }

    @Test
    void rejectsVolumesOutOfRange() {
        assertThrows(IllegalArgumentException.class,
                () -> new GameSettings(1920, 1080, true, false, 1.5f, 0.5f, 0.5f));
        assertThrows(IllegalArgumentException.class,
                () -> new GameSettings(1920, 1080, true, false, 0.5f, -0.1f, 0.5f));
        assertThrows(IllegalArgumentException.class,
                () -> new GameSettings(1920, 1080, true, false, 0.5f, 0.5f, 2f));
    }

    @Test
    void acceptsVolumesAtBoundaries() {
        final GameSettings low = new GameSettings(1, 1, false, false, 0f, 0f, 0f);
        final GameSettings high = new GameSettings(1, 1, false, false, 1f, 1f, 1f);
        assertEquals(0f, low.getOverallVolume());
        assertEquals(1f, high.getEffectsVolume());
    }
}
