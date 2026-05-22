package cz.empatix.audio.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

class AudioSettingsTest {

    @Test
    void defaultsAreFullVolume() {
        final AudioSettings s = new AudioSettings();
        assertEquals(1f, s.getOverallVolume(), 1e-6);
        assertEquals(1f, s.getSoundtrackVolume(), 1e-6);
        assertEquals(1f, s.getSoundEffectVolume(), 1e-6);
    }

    @Test
    void settersRoundTrip() {
        final AudioSettings s = new AudioSettings();
        s.setOverallVolume(0.5f);
        s.setSoundtrackVolume(0.25f);
        s.setSoundEffectVolume(0.75f);
        assertEquals(0.5f, s.getOverallVolume(), 1e-6);
        assertEquals(0.25f, s.getSoundtrackVolume(), 1e-6);
        assertEquals(0.75f, s.getSoundEffectVolume(), 1e-6);
    }

    @Test
    void getInstanceReturnsSameSingleton() {
        assertSame(AudioSettings.getInstance(), AudioSettings.getInstance());
    }
}
