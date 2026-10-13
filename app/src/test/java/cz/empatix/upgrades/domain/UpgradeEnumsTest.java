package cz.empatix.upgrades.domain;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

/**
 * Direct value-logic coverage of the upgrade domain enums.
 */
class UpgradeEnumsTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    @Test
    void qualityLevelHasThreeTiers() {
        assertEquals(3, QualityLevel.values().length);
        assertSame(QualityLevel.COMMON, QualityLevel.valueOf("COMMON"));
        assertSame(QualityLevel.RARE, QualityLevel.valueOf("RARE"));
        assertSame(QualityLevel.EPIC, QualityLevel.valueOf("EPIC"));
    }

    @Test
    void treeTypeHasThreeKinds() {
        assertEquals(3, TreeType.values().length);
        assertSame(TreeType.OFFENSIVE, TreeType.valueOf("OFFENSIVE"));
        assertSame(TreeType.DEFENSIVE, TreeType.valueOf("DEFENSIVE"));
        assertSame(TreeType.UTILITY, TreeType.valueOf("UTILITY"));
    }

    @Test
    void playerUpgradeNameContainsAllKnownTalentsPlusUnknown() {
        // 15 talents (5 per tree) + UNKNOWN sentinel.
        assertEquals(16, PlayerUpgradeName.values().length);
        assertSame(PlayerUpgradeName.UNKNOWN, PlayerUpgradeName.valueOf("UNKNOWN"));
        assertSame(PlayerUpgradeName.IMPROVED_MOVEMENT_SPEED, PlayerUpgradeName.valueOf("IMPROVED_MOVEMENT_SPEED"));
        assertSame(PlayerUpgradeName.PET, PlayerUpgradeName.valueOf("PET"));
    }
}
