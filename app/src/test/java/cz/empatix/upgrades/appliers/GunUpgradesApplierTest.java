package cz.empatix.upgrades.appliers;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.RngService;
import cz.empatix.guns.domain.Gun;
import cz.empatix.guns.service.GunDamageEventHook;
import cz.empatix.guns.service.GunService;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.ArgumentCaptor;
import org.mockito.MockedStatic;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Logic coverage of {@link GunUpgradesApplier}: damage-modifier hooks, ammo-capacity scaling and
 * extra-slot unlock. Collaborators are mocked; damage hooks are captured and invoked to check the
 * deterministic multipliers (the critical-hit hook is only asserted to be a no-op on its
 * non-critical path / valid output range because it draws randomness).
 *
 * <p>{@link GunService} has a static initializer that fetches a HUD texture through
 * {@link AssetsService}; mocking the class triggers that block, so we force class-init once under a
 * {@link MockedAssets}-stubbed static before any {@code mock(GunService.class)} call.
 */
class GunUpgradesApplierTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            // Force GunService's asset-fetching static initializer to run while assets are stubbed.
            try {
                Class.forName("cz.empatix.guns.service.GunService", true,
                        GunUpgradesApplierTest.class.getClassLoader());
            } catch (final ClassNotFoundException e) {
                throw new IllegalStateException(e);
            }
        }
    }

    @Test
    void nothingBoughtTouchesNoGunState() {
        final GunService gunService = mock(GunService.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);

        new GunUpgradesApplier(gunService, upgrades, new RngService(0L)).apply();

        verify(gunService, never()).addHook(any());
        verify(gunService, never()).addSlot();
    }

    @Test
    void improvedDamageHookScalesDamageBy120Percent() {
        final GunService gunService = mock(GunService.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        when(upgrades.isBought(PlayerUpgradeName.IMPROVED_DAMAGE)).thenReturn(true);

        new GunUpgradesApplier(gunService, upgrades, new RngService(0L)).apply();

        final ArgumentCaptor<GunDamageEventHook> captor = ArgumentCaptor.forClass(GunDamageEventHook.class);
        verify(gunService).addHook(captor.capture());
        assertEquals(120, captor.getValue().apply(100)); // (int)(100 * 1.2)
    }

    @Test
    void enhancedDamageHookScalesDamageBy125Percent() {
        final GunService gunService = mock(GunService.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        when(upgrades.isBought(PlayerUpgradeName.ENHANCED_DAMAGE)).thenReturn(true);

        new GunUpgradesApplier(gunService, upgrades, new RngService(0L)).apply();

        final ArgumentCaptor<GunDamageEventHook> captor = ArgumentCaptor.forClass(GunDamageEventHook.class);
        verify(gunService).addHook(captor.capture());
        assertEquals(125, captor.getValue().apply(100)); // (int)(100 * 1.25)
    }

    @Test
    void criticalHitHookReturnsEitherBaseOrDoubleDamage() {
        final GunService gunService = mock(GunService.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        when(upgrades.isBought(PlayerUpgradeName.CRITICAL_HIT)).thenReturn(true);

        new GunUpgradesApplier(gunService, upgrades, new RngService(0L)).apply();

        final ArgumentCaptor<GunDamageEventHook> captor = ArgumentCaptor.forClass(GunDamageEventHook.class);
        verify(gunService).addHook(captor.capture());

        // Deterministic invariant despite the 10% RNG: result is always base or exactly double.
        for (int i = 0; i < 200; i++) {
            final int out = captor.getValue().apply(50);
            assertTrue(out == 50 || out == 100, "crit hook produced unexpected value: " + out);
        }
    }

    @Test
    void additionalAmmoScalesEveryGunsMaxAmmoBy110Percent() {
        final GunService gunService = mock(GunService.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        when(upgrades.isBought(PlayerUpgradeName.ADDITIONAL_AMMO)).thenReturn(true);

        final Gun gun = mock(Gun.class);
        when(gun.getMaxAmmo()).thenReturn(50);
        when(gunService.getGuns()).thenReturn(List.of(gun));

        new GunUpgradesApplier(gunService, upgrades, new RngService(0L)).apply();

        verify(gun).setMaxAmmo(55); // (int)(50 * 1.1)
    }

    @Test
    void additionalGunUnlocksAnExtraSlot() {
        final GunService gunService = mock(GunService.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        when(upgrades.isBought(PlayerUpgradeName.ADDITIONAL_GUN)).thenReturn(true);

        new GunUpgradesApplier(gunService, upgrades, new RngService(0L)).apply();

        verify(gunService).addSlot();
    }
}
