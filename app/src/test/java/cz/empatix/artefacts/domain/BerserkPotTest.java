package cz.empatix.artefacts.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.MoveForceEventHook;
import cz.empatix.player.Player;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.verify;

/**
 * Covers the move-force buff lifecycle of {@link BerserkPot}: activation registers a
 * {@link MoveForceEventHook} on the player. The constructor builds a HUD image and a particle
 * service, so the test runs under a mocked {@link AssetsService}.
 */
class BerserkPotTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    @Test
    void activateAddsAMoveForceHook() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Player player = mock(Player.class);
            final BerserkPot artefact = new BerserkPot(player);

            artefact.activate();

            verify(player).addHook(any(MoveForceEventHook.class));
        }
    }

    @Test
    void exposesTexturePathHudIconAndUpdatesWithoutThrowing() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final BerserkPot artefact = new BerserkPot(mock(Player.class));

            assertEquals("ingame/artefacts/berserkpot.tga", artefact.getTexturePath());
            assertEquals(1, artefact.hudToRender().size());

            // No particles spawned yet -> render list is empty but non-null, and update is safe.
            artefact.update(0.016);
            assertNotNull(artefact.getRenderables());
        }
    }
}
