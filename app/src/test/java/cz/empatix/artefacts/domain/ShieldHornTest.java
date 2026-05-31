package cz.empatix.artefacts.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.DamageEventHook;
import cz.empatix.player.Player;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.ArgumentCaptor;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Covers the shield lifecycle of {@link ShieldHorn}: activation registers a damage hook, and
 * {@code fixedUpdate} unregisters it once the hook has absorbed a hit (gone inactive).
 */
class ShieldHornTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    private Player newPlayer() {
        final Player player = mock(Player.class);
        when(player.getRenderPosition()).thenReturn(new Vector2f(10, 20));
        when(player.getWidth()).thenReturn(32);
        when(player.getHeight()).thenReturn(32);
        return player;
    }

    @Test
    void activateRegistersADamageHook() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Player player = newPlayer();
            final ShieldHorn artefact = new ShieldHorn(player);

            artefact.activate();

            verify(player).addDamageHook(any(ShieldDamageEventHook.class));
        }
    }

    @Test
    void shieldIsRenderedWhileHookIsActiveAndHiddenOtherwise() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final ShieldHorn artefact = new ShieldHorn(newPlayer());

            // Before activation there is no shield to render.
            assertTrue(artefact.getRenderables().isEmpty());

            artefact.activate();
            // Active hook -> the magic shield is rendered.
            assertEquals(1, artefact.getRenderables().size());
        }
    }

    @Test
    void fixedUpdateRemovesHookOnceItHasAbsorbedDamage() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Player player = newPlayer();
            final ShieldHorn artefact = new ShieldHorn(player);
            artefact.activate();

            final ArgumentCaptor<DamageEventHook> captor = ArgumentCaptor.forClass(DamageEventHook.class);
            verify(player).addDamageHook(captor.capture());
            final DamageEventHook hook = captor.getValue();

            // Still active -> fixedUpdate keeps the hook registered.
            artefact.fixedUpdate(0.016);
            verify(player, never()).removeDamageHook(hook);

            // The hook absorbs a hit and becomes inactive.
            hook.onDamage(10);
            artefact.fixedUpdate(0.016);
            verify(player).removeDamageHook(hook);

            // With the hook gone the shield is no longer rendered.
            assertTrue(artefact.getRenderables().isEmpty());
        }
    }

    @Test
    void updateRepositionsTheShieldWithoutThrowing() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final ShieldHorn artefact = new ShieldHorn(newPlayer());

            artefact.update(0.016);

            assertEquals("ingame/artefacts/shieldhorn.tga", artefact.getTexturePath());
            assertEquals(1, artefact.hudToRender().size());
        }
    }
}
