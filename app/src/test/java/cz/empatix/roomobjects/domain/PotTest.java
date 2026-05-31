package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Headless domain-logic coverage for the real {@link Pot}: damage/death semantics, the enemy-immune
 * rule, drop-once guard, immovability toggle and position/dimension getters.
 */
class PotTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    @Test
    void playerHitDamagesAndDeathThresholdKills() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pot pot = new Pot(mock(ItemService.class));

            assertEquals(2, pot.hit(DamageSourceType.PLAYER, 2));
            assertFalse(pot.isDead());
            pot.hit(DamageSourceType.PLAYER, 2);
            assertTrue(pot.isDead());
        }
    }

    @Test
    void trapDamageAlsoCountsButEnemyDoesNot() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pot pot = new Pot(mock(ItemService.class));

            assertEquals(0, pot.hit(DamageSourceType.ENEMY, 4));
            assertFalse(pot.isDead());
            assertEquals(4, pot.hit(DamageSourceType.TRAP, 4));
            assertTrue(pot.isDead());
        }
    }

    @Test
    void itemDroppedOnceDuringDeathAnimation() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemService itemService = mock(ItemService.class);
            final ItemFactory itemFactory = mock(ItemFactory.class);
            when(itemService.getItemFactory()).thenReturn(itemFactory);
            when(itemFactory.createItemDrop()).thenReturn(mock(ItemDrop.class));
            final Pot pot = new Pot(itemService);

            pot.hit(DamageSourceType.PLAYER, 4);
            for (int i = 0; i < 8; i++) {
                pot.update(0.016);
            }

            verify(itemService, times(1)).add(any(ItemDrop.class));
        }
    }

    @Test
    void setImmovableDoesNotThrow() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pot pot = new Pot(mock(ItemService.class));

            pot.setImmovable();
            pot.setPosition(10f, 20f);

            assertEquals(10f, pot.getPosition().x, 0.0001f);
            assertEquals(20f, pot.getPosition().y, 0.0001f);
            assertEquals(64 * 3, pot.getWidth());
            assertEquals(64 * 3, pot.getHeight());
            assertFalse(pot.canRemove());
        }
    }
}
