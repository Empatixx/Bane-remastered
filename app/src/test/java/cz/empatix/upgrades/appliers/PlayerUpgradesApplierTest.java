package cz.empatix.upgrades.appliers;

import cz.empatix.player.DamageEventHook;
import cz.empatix.player.MoveForceEventHook;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.player.Player;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.ArgumentCaptor;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Logic coverage of {@link PlayerUpgradesApplier}: each bought talent applies the correct stat
 * change / hook. Collaborators are mocked; hook lambdas are captured and invoked to assert the
 * concrete modifier math (e.g. -10% damage, +10% move force, the health/armour multipliers).
 */
class PlayerUpgradesApplierTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    @Test
    void noUpgradesBoughtAppliesNothing() {
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        final Player player = mock(Player.class); // all isBought default to false

        new PlayerUpgradesApplier(upgrades, player).apply();

        verify(player, never()).setMaxHealth(anyInt());
        verify(player, never()).setMaxArmor(anyInt());
        verify(player, never()).addDamageHook(org.mockito.ArgumentMatchers.any());
        verify(player, never()).addHook(org.mockito.ArgumentMatchers.any());
    }

    @Test
    void improvedHealthScalesMaxHealthBy120Percent() {
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        final Player player = mock(Player.class);
        when(upgrades.isBought(PlayerUpgradeName.IMPROVED_HEALTH)).thenReturn(true);
        when(player.getMaxHealth()).thenReturn(100);

        new PlayerUpgradesApplier(upgrades, player).apply();

        verify(player).setMaxHealth(120); // (int)(100 * 1.2)
    }

    @Test
    void improvedArmorScalesMaxArmorBy120Percent() {
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        final Player player = mock(Player.class);
        when(upgrades.isBought(PlayerUpgradeName.IMPROVED_ARMOR)).thenReturn(true);
        when(player.getMaxArmor()).thenReturn(30);

        new PlayerUpgradesApplier(upgrades, player).apply();

        verify(player).setMaxArmor(36); // (int)(30 * 1.2)
    }

    @Test
    void enhancedHealthScalesMaxHealthBy125Percent() {
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        final Player player = mock(Player.class);
        when(upgrades.isBought(PlayerUpgradeName.ENHANCED_HEALTH)).thenReturn(true);
        when(player.getMaxHealth()).thenReturn(200);

        new PlayerUpgradesApplier(upgrades, player).apply();

        verify(player).setMaxHealth(250); // (int)(200 * 1.25)
    }

    @Test
    void reducedDamageTakenHookCutsDamageBy10Percent() {
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        final Player player = mock(Player.class);
        when(upgrades.isBought(PlayerUpgradeName.REDUCED_DAMAGE_TAKEN)).thenReturn(true);

        new PlayerUpgradesApplier(upgrades, player).apply();

        final ArgumentCaptor<DamageEventHook> captor = ArgumentCaptor.forClass(DamageEventHook.class);
        verify(player).addDamageHook(captor.capture());
        assertEquals(90, captor.getValue().onDamage(100)); // (int)(100 * 0.9)
        assertEquals(9, captor.getValue().onDamage(10));   // (int)(10 * 0.9)
    }

    @Test
    void improvedMovementSpeedHookAddsTenPercentForce() {
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        final Player player = mock(Player.class);
        when(upgrades.isBought(PlayerUpgradeName.IMPROVED_MOVEMENT_SPEED)).thenReturn(true);

        new PlayerUpgradesApplier(upgrades, player).apply();

        final ArgumentCaptor<MoveForceEventHook> captor = ArgumentCaptor.forClass(MoveForceEventHook.class);
        verify(player).addHook(captor.capture());
        assertEquals(110L, captor.getValue().onMoveForce(100)); // (int)(100 * 1.1)
    }
}
