package cz.empatix.artefacts.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.Player;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.when;

/**
 * Covers the burst behaviour of {@link RingOfDeath}: activation spawns a fixed ring of bullets that
 * surface through {@code getRenderables}. Bullet construction needs both assets and a physics world.
 */
class RingOfDeathTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private Player playerAt(final float x, final float y) {
        final Player player = mock(Player.class);
        when(player.getPosition()).thenReturn(new Vector2f(x, y));
        return player;
    }

    @Test
    void noBulletsBeforeActivation() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final RingOfDeath artefact = new RingOfDeath(playerAt(100, 100));

            assertTrue(artefact.getRenderables().isEmpty());
            assertEquals("ingame/artefacts/ringofdeath.tga", artefact.getTexturePath());
            assertEquals(1, artefact.hudToRender().size());
        }
    }

    @Test
    void activateSpawnsFiftyBullets() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final RingOfDeath artefact = new RingOfDeath(playerAt(200, 150));

            artefact.activate();

            assertEquals(50, artefact.getRenderables().size());
        }
    }

    @Test
    void updateKeepsFreshBulletsAlive() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final RingOfDeath artefact = new RingOfDeath(playerAt(0, 0));
            artefact.activate();

            // A single update tick cannot complete a collision animation, so none are pruned.
            artefact.update(0.016);
            artefact.fixedUpdate(0.016);

            assertEquals(50, artefact.getRenderables().size());
        }
    }
}
