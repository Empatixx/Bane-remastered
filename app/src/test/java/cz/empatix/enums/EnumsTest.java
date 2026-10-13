package cz.empatix.enums;

import cz.empatix.affixes.domain.AffixType;
import cz.empatix.animation.AnimationState;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.enemies.domain.EnemyType;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.guns.domain.GunType;
import cz.empatix.keyboard.domain.KeyboardType;
import cz.empatix.minimap.domain.BigMinimapRenderType;
import cz.empatix.roomobjects.domain.ChestType;
import cz.empatix.roomobjects.domain.PathGateDirection;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.sound.SoundEffect;
import cz.empatix.sound.Soundtrack;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import cz.empatix.upgrades.domain.QualityLevel;
import cz.empatix.upgrades.domain.TreeType;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

/** Covers the pure / simple enums: counts and values()/valueOf round-trips. */
class EnumsTest {

    @Test
    void gunTypeRoundTrip() {
        assertEquals(5, GunType.values().length);
        for (final GunType t : GunType.values()) {
            assertSame(t, GunType.valueOf(t.name()));
        }
    }

    @Test
    void enemyTypeRoundTrip() {
        assertEquals(2, EnemyType.values().length);
        assertSame(EnemyType.COMMON, EnemyType.valueOf("COMMON"));
        assertSame(EnemyType.BOSS, EnemyType.valueOf("BOSS"));
    }

    @Test
    void roomTypeRoundTrip() {
        assertEquals(8, RoomType.values().length);
        for (final RoomType t : RoomType.values()) {
            assertSame(t, RoomType.valueOf(t.name()));
        }
    }

    @Test
    void roomStateRoundTrip() {
        assertEquals(2, RoomState.values().length);
        assertSame(RoomState.OPEN, RoomState.valueOf("OPEN"));
        assertSame(RoomState.CLOSED, RoomState.valueOf("CLOSED"));
    }

    @Test
    void playerUpgradeNameRoundTrip() {
        assertEquals(16, PlayerUpgradeName.values().length);
        for (final PlayerUpgradeName n : PlayerUpgradeName.values()) {
            assertSame(n, PlayerUpgradeName.valueOf(n.name()));
        }
    }

    @Test
    void qualityLevelRoundTrip() {
        assertEquals(3, QualityLevel.values().length);
        for (final QualityLevel q : QualityLevel.values()) {
            assertSame(q, QualityLevel.valueOf(q.name()));
        }
    }

    @Test
    void treeTypeRoundTrip() {
        assertEquals(3, TreeType.values().length);
        for (final TreeType t : TreeType.values()) {
            assertSame(t, TreeType.valueOf(t.name()));
        }
    }

    @Test
    void gameStateTypeRoundTrip() {
        assertEquals(4, GameStateType.values().length);
        for (final GameStateType t : GameStateType.values()) {
            assertSame(t, GameStateType.valueOf(t.name()));
        }
    }

    @Test
    void cursorTypeRoundTrip() {
        assertEquals(4, CursorType.values().length);
        for (final CursorType t : CursorType.values()) {
            assertSame(t, CursorType.valueOf(t.name()));
        }
    }

    @Test
    void chestTypeRoundTrip() {
        assertEquals(2, ChestType.values().length);
        assertSame(ChestType.WEAPON, ChestType.valueOf("WEAPON"));
        assertSame(ChestType.ARTEFACT, ChestType.valueOf("ARTEFACT"));
    }

    @Test
    void pathGateDirectionRoundTrip() {
        assertEquals(3, PathGateDirection.values().length);
        for (final PathGateDirection d : PathGateDirection.values()) {
            assertSame(d, PathGateDirection.valueOf(d.name()));
        }
    }

    @Test
    void affixTypeRoundTrip() {
        assertEquals(5, AffixType.values().length);
        for (final AffixType a : AffixType.values()) {
            assertSame(a, AffixType.valueOf(a.name()));
        }
    }

    @Test
    void animationStateRoundTripAndMarkerInterface() {
        for (final AnimationState s : AnimationState.values()) {
            assertSame(s, AnimationState.valueOf(s.name()));
        }
        // implements the render-engine marker interface
        assertTrue(cz.empatix.render.domain.AnimationState.class
                .isAssignableFrom(AnimationState.class));
    }

    @Test
    void keyboardTypeBuildsTexturePath() {
        assertEquals("ingame/keyboard/q.tga", KeyboardType.Q.getTexturePath());
        assertEquals("ingame/keyboard/tab.tga", KeyboardType.TAB.getTexturePath());
        assertEquals("ingame/keyboard/capslock.tga", KeyboardType.CAPS_LOCK.getTexturePath());
        assertEquals("ingame/keyboard/1.tga", KeyboardType.NUMBER_ONE.getTexturePath());
        assertEquals("ingame/keyboard/5.tga", KeyboardType.NUMBER_FIVE.getTexturePath());
        // every constant produces a non-empty .tga path under the expected folder
        for (final KeyboardType k : KeyboardType.values()) {
            final String path = k.getTexturePath();
            assertTrue(path.startsWith("ingame/keyboard/"), path);
            assertTrue(path.endsWith(".tga"), path);
        }
    }

    @Test
    void bigMinimapRenderTypeAlphaPerConstant() {
        assertEquals(1f, BigMinimapRenderType.CURRENT.alpha());
        assertEquals(0.75f, BigMinimapRenderType.VISITED.alpha());
        assertEquals(0.5f, BigMinimapRenderType.UNVISITED.alpha());
        assertEquals(0f, BigMinimapRenderType.UNSEEN.alpha());
    }

    @Test
    void soundtrackGetPath() {
        assertEquals("intro.ogg", Soundtrack.MAIN_MENU.getPath());
        assertEquals("audio.ogg", Soundtrack.INGAME.getPath());
        assertEquals("ambient.ogg", Soundtrack.PROGRESS_ROOM.getPath());
        assertEquals("boss.ogg", Soundtrack.BOSS_ROOM.getPath());
    }

    @Test
    void soundEffectSingleVariantPathIsStable() {
        // a single-path effect always returns the same value
        for (int i = 0; i < 50; i++) {
            assertEquals("buy.ogg", SoundEffect.BUY.getPath());
            assertEquals("guns/M4_shoot.ogg", SoundEffect.M4_SHOOT.getPath());
        }
    }

    @Test
    void soundEffectMultiVariantStaysAmongAllowedVariants() {
        // PLAYER_HURT has two variants; over many calls every result must be one of them
        final java.util.Set<String> allowed =
                java.util.Set.of("playerhurt_1.ogg", "playerhurt_2.ogg");
        for (int i = 0; i < 200; i++) {
            assertTrue(allowed.contains(SoundEffect.PLAYER_HURT.getPath()));
        }
    }
}
