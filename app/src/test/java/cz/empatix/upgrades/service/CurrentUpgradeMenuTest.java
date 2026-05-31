package cz.empatix.upgrades.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.QualityLevel;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Purchase-flow logic of {@link CurrentUpgradeMenu}. Construction builds CommonText/TransparentButton
 * widgets that fetch fonts; with {@link MockedAssets} the font is a mock (zero-size metrics), which is
 * fine for the click-math here. We exercise the {@code setCurrent}/{@code mousePressed} state machine
 * and assert it mutates the bought-state of the selected upgrade. The Buy button is positioned at
 * (735,410); with zero-width text the hit box is a small square centred there, so we click that point.
 */
class CurrentUpgradeMenuTest {

    private static final float BUY_X = 735f;
    private static final float BUY_Y = 410f;

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    private static PlayerUpgrade buyableUpgrade() {
        // No parent -> canBuy() true; not bought -> Buy path is active.
        final PlayerUpgrade u = new PlayerUpgrade("ingame/talents/boot.tga", QualityLevel.COMMON, false);
        u.setTitle("Swiftwind Steps");
        u.setDescription("Increases movement speed\n by 10%");
        u.setPrice(20);
        return u;
    }

    @Test
    void mousePressedWithNoSelectionIsANoOp() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final CurrentUpgradeMenu menu = new CurrentUpgradeMenu();

            // currentUpgrade is null -> returns false, nothing happens.
            assertFalse(menu.mousePressed(0, BUY_X, BUY_Y));
        }
    }

    @Test
    void clickingBuyMarksTheSelectedUpgradeBought() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final CurrentUpgradeMenu menu = new CurrentUpgradeMenu();
            final PlayerUpgrade upgrade = buyableUpgrade();

            menu.setCurrent(upgrade);
            assertFalse(upgrade.isBought());

            // update() lets the Buy button compute its hover state at the click point first.
            menu.update(0.016, BUY_X, BUY_Y);
            final boolean consumed = menu.mousePressed(0, BUY_X, BUY_Y);

            assertTrue(consumed);            // click landed on the hovered Buy button
            assertTrue(upgrade.isBought());  // purchase applied to the live upgrade
        }
    }

    @Test
    void clickingAwayFromBuyDoesNotPurchase() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final CurrentUpgradeMenu menu = new CurrentUpgradeMenu();
            final PlayerUpgrade upgrade = buyableUpgrade();
            menu.setCurrent(upgrade);

            // Far from the Buy hit box -> not hovered, not purchased.
            final boolean consumed = menu.mousePressed(0, 0, 0);

            assertFalse(consumed);
            assertFalse(upgrade.isBought());
        }
    }

    @Test
    void alreadyBoughtUpgradeCannotBeRepurchased() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final CurrentUpgradeMenu menu = new CurrentUpgradeMenu();
            final PlayerUpgrade upgrade = buyableUpgrade();
            upgrade.setBought(true);
            menu.setCurrent(upgrade);

            // Bought upgrades short-circuit mousePressed -> false, no Buy interaction.
            assertFalse(menu.mousePressed(0, BUY_X, BUY_Y));
            assertTrue(upgrade.isBought());
        }
    }

    @Test
    void lockedUpgradeCannotBeBought() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final CurrentUpgradeMenu menu = new CurrentUpgradeMenu();
            final PlayerUpgrade parent = buyableUpgrade();
            final PlayerUpgrade child = buyableUpgrade();
            child.setParent(parent); // parent not bought -> canBuy() false

            menu.setCurrent(child);
            menu.update(0.016, BUY_X, BUY_Y);

            // Not buyable -> mousePressed returns false and no purchase happens even on the Buy point.
            assertFalse(menu.mousePressed(0, BUY_X, BUY_Y));
            assertFalse(child.isBought());
        }
    }

    @Test
    void setCurrentNullClearsSelectionWithoutError() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final CurrentUpgradeMenu menu = new CurrentUpgradeMenu();
            menu.setCurrent(buyableUpgrade());
            menu.setCurrent(null); // early-return branch

            assertFalse(menu.mousePressed(0, BUY_X, BUY_Y));
        }
    }
}
