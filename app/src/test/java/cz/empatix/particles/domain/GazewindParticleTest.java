package cz.empatix.particles.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

/**
 * Headless coverage for the {@link GazewindParticle} record: a small animated SCENE sprite. The
 * texture comes from the GL-free mock (see {@link HeadlessAssets}); we assert the fixed 32x32
 * footprint (16px sprite at SCALE 2), the scene/z-index, position round-trip, and that a fresh
 * particle has not yet finished playing. The GL draw path ({@code renderData}) is not exercised.
 */
@ExtendWith(HeadlessAssets.class)
class GazewindParticleTest {

    private static final int EXPECTED_SIZE = 16 * 2; // DIMENSIONS = 16 * SCALE(2)

    @Test
    void footprintIsTheScaledSpriteSize() {
        final GazewindParticle particle = new GazewindParticle("ingame/gazewind_particle.tga");

        assertEquals(EXPECTED_SIZE, particle.getWidth());
        assertEquals(EXPECTED_SIZE, particle.getHeight());
    }

    @Test
    void rendersOnTheSceneLayerAtZIndexOne() {
        final GazewindParticle particle = new GazewindParticle("ingame/gazewind_particle.tga");

        assertEquals(RenderScene.SCENE, particle.renderScene());
        assertEquals(1, particle.getZIndex());
    }

    @Test
    void positionRoundTrips() {
        final GazewindParticle particle = new GazewindParticle("ingame/gazewind_particle.tga");

        particle.setPosition(123f, 456f);

        assertEquals(123f, particle.transform().x, 0.0001f);
        assertEquals(456f, particle.transform().y, 0.0001f);
    }

    @Test
    void freshParticleHasNotFinishedPlaying() {
        final GazewindParticle particle = new GazewindParticle("ingame/gazewind_particle.tga");

        assertFalse(particle.shouldRemove(), "a newly spawned particle is not yet removable");
    }

    @Test
    void updateBeforeTheFrameDelayDoesNotFinishTheAnimation() {
        final GazewindParticle particle = new GazewindParticle("ingame/gazewind_particle.tga");

        // back-to-back updates fall inside the 100ms frame delay -> still mid-play
        particle.update(0.016f);
        particle.update(0.016f);

        assertFalse(particle.shouldRemove());
    }
}
