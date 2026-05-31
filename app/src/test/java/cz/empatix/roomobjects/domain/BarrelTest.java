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
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.Mockito.atLeastOnce;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Headless domain-logic coverage for the real {@link Barrel}: damage accumulation, the death
 * threshold, the player-only damage rule and the drop-once guard that fires during the DEATH
 * animation in {@link Barrel#update(double)}.
 */
class BarrelTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private static ItemService itemServiceDroppingMock() {
        final ItemService itemService = mock(ItemService.class);
        final ItemFactory itemFactory = mock(ItemFactory.class);
        final ItemDrop itemDrop = mock(ItemDrop.class);
        when(itemService.getItemFactory()).thenReturn(itemFactory);
        when(itemFactory.createItemDrop()).thenReturn(itemDrop);
        return itemService;
    }

    @Test
    void playerHitReducesHealthButDoesNotKillBeforeThreshold() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Barrel barrel = new Barrel(mock(ItemService.class));

            final int returned = barrel.hit(DamageSourceType.PLAYER, 1);

            assertEquals(1, returned);
            assertFalse(barrel.isDead());
        }
    }

    @Test
    void enemyDamageIsIgnored() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Barrel barrel = new Barrel(mock(ItemService.class));

            final int returned = barrel.hit(DamageSourceType.ENEMY, 99);

            assertEquals(0, returned);
            assertFalse(barrel.isDead());
        }
    }

    @Test
    void enoughDamageKillsTheBarrel() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Barrel barrel = new Barrel(mock(ItemService.class));

            barrel.hit(DamageSourceType.PLAYER, 4);

            assertTrue(barrel.isDead());
        }
    }

    @Test
    void hittingADeadBarrelReturnsZeroAndDoesNotOverkill() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Barrel barrel = new Barrel(mock(ItemService.class));
            barrel.hit(DamageSourceType.PLAYER, 4);

            final int returned = barrel.hit(DamageSourceType.PLAYER, 4);

            assertEquals(0, returned);
            assertTrue(barrel.isDead());
        }
    }

    @Test
    void itemIsDroppedExactlyOnceWhileDeathAnimationRuns() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemService itemService = itemServiceDroppingMock();
            final Barrel barrel = new Barrel(itemService);

            barrel.hit(DamageSourceType.PLAYER, 4); // -> DEATH state
            // Several update ticks: drop guard must fire on the first DEATH tick only.
            for (int i = 0; i < 10; i++) {
                barrel.update(0.016);
            }

            verify(itemService, times(1)).add(any(ItemDrop.class));
            verify(itemService, atLeastOnce()).getItemFactory();
        }
    }

    @Test
    void positionReflectsSetPosition() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Barrel barrel = new Barrel(mock(ItemService.class));

            barrel.setPosition(120f, 240f);

            assertEquals(120f, barrel.getPosition().x, 0.0001f);
            assertEquals(240f, barrel.getPosition().y, 0.0001f);
        }
    }

    @Test
    void dimensionsAreDerivedFromScaleConstants() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Barrel barrel = new Barrel(mock(ItemService.class));

            assertEquals(64 * 3, barrel.getWidth());
            assertEquals(64 * 3, barrel.getHeight());
            assertEquals(19 * 3, barrel.getCollissionWidth());
            assertEquals(21 * 3, barrel.getCollissionHeight());
            assertFalse(barrel.canRemove());
        }
    }

    @Test
    void itemDropDropPositionIsConfigured() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemService itemService = mock(ItemService.class);
            final ItemFactory itemFactory = mock(ItemFactory.class);
            final ItemDrop itemDrop = mock(ItemDrop.class);
            when(itemService.getItemFactory()).thenReturn(itemFactory);
            when(itemFactory.createItemDrop()).thenReturn(itemDrop);
            final Barrel barrel = new Barrel(itemService);

            barrel.hit(DamageSourceType.PLAYER, 4);
            barrel.update(0.016);

            verify(itemDrop).setPosition(anyFloat(), anyFloat());
        }
    }
}
