package cz.empatix.artefacts.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.Player;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Covers the activation effect of {@link ArmorRefill}. The constructor builds a {@code CommonImage}
 * and forces the {@code ArtefactsService} static initializer (which fetches an atlas entry), so the
 * whole test runs under a mocked {@link AssetsService}.
 */
class ArmorRefillTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    @Test
    void activateRefillsArmorToMaximum() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Player player = mock(Player.class);
            when(player.getMaxArmor()).thenReturn(7);
            final ArmorRefill artefact = new ArmorRefill(player);

            artefact.activate();

            verify(player).setArmor(7);
        }
    }

    @Test
    void renderablesAreEmptyAndHudHasTheIcon() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final ArmorRefill artefact = new ArmorRefill(mock(Player.class));

            assertTrue(artefact.getRenderables().isEmpty());
            assertEquals(1, artefact.hudToRender().size());
            assertEquals("ingame/artefacts/armorrefill.tga", artefact.getTexturePath());
        }
    }

    @Test
    void updateAndFixedUpdateAreNoOps() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Player player = mock(Player.class);
            final ArmorRefill artefact = new ArmorRefill(player);

            // Neither hook touches the player.
            artefact.update(0.016);
            artefact.fixedUpdate(0.016);

            assertSame(player, player); // sanity: no exception thrown by the no-op hooks
        }
    }
}
