package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the real {@link Spike} ({@code DamageSource} + {@code Trap}): the trap type,
 * the frame-driven damage window in {@link Spike#update(double)}, and the deactivate path which
 * parks the spike at a retracted frame so it deals no damage on the following tick.
 */
class SpikeTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    @Test
    void typeIsTrapAndDamageStartsActive() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Spike spike = new Spike();

            assertEquals(DamageSourceType.TRAP, spike.type());
            // Frame 0 is in the "active" window, so the first tick arms the spike.
            spike.update(0.016);
            assertEquals(1, spike.damage());
            assertFalse(spike.canRemove());
        }
    }

    @Test
    void deactivatedSpikeRetractsAndDealsNoDamage() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Spike spike = new Spike();

            spike.deactivate(); // parks at frame 2 (retracted)
            spike.update(0.016); // damage = frameIndex >= 2 ? 0 : 1

            assertEquals(0, spike.damage());
        }
    }

    @Test
    void reactivateRestoresActiveAnimation() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Spike spike = new Spike();
            spike.deactivate();

            spike.activate();
            spike.update(0.016);

            assertEquals(1, spike.damage());
        }
    }

    @Test
    void geometryPositionAndPhysicsBodyExposed() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Spike spike = new Spike();
            spike.setPosition(60f, 70f);

            assertEquals(16 * 4, spike.getWidth());
            assertEquals(16 * 4, spike.getHeight());
            assertEquals(60f, spike.getPosition().x, 0.0001f);
            assertEquals(70f, spike.getPosition().y, 0.0001f);
            assertNotNull(spike.getPhysicsObject());
            assertSame(spike, spike.getPhysicsObject().getUserData());
        }
    }
}
