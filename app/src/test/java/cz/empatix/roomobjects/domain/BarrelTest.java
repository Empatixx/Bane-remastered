package cz.empatix.roomobjects.domain;

import cz.empatix.collision.DamageSourceType;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.Mockito.atLeastOnce;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Headless domain-logic coverage for the real {@link Barrel}: damage accumulation, the death
 * threshold, the player-only damage rule and the drop-once guard that fires during the DEATH
 * animation in {@link Barrel#update(double)}.
 */
@ExtendWith(HeadlessAssets.class)
class BarrelTest {

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
        final Barrel barrel = new Barrel(mock(ItemService.class));

        final int returned = barrel.hit(DamageSourceType.PLAYER, 1);

        assertEquals(1, returned);
        assertFalse(barrel.isDead());
    }

    @Test
    void enemyDamageIsIgnored() {
        final Barrel barrel = new Barrel(mock(ItemService.class));

        final int returned = barrel.hit(DamageSourceType.ENEMY, 99);

        assertEquals(0, returned);
        assertFalse(barrel.isDead());
    }

    @Test
    void enoughDamageKillsTheBarrel() {
        final Barrel barrel = new Barrel(mock(ItemService.class));

        barrel.hit(DamageSourceType.PLAYER, 4);

        assertTrue(barrel.isDead());
    }

    @Test
    void hittingADeadBarrelReturnsZeroAndDoesNotOverkill() {
        final Barrel barrel = new Barrel(mock(ItemService.class));
        barrel.hit(DamageSourceType.PLAYER, 4);

        final int returned = barrel.hit(DamageSourceType.PLAYER, 4);

        assertEquals(0, returned);
        assertTrue(barrel.isDead());
    }

    @Test
    void itemIsDroppedExactlyOnceWhileDeathAnimationRuns() {
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

    @Test
    void positionReflectsSetPosition() {
        final Barrel barrel = new Barrel(mock(ItemService.class));

        barrel.setPosition(120f, 240f);

        assertEquals(120f, barrel.getPosition().x, 0.0001f);
        assertEquals(240f, barrel.getPosition().y, 0.0001f);
    }

    @Test
    void dimensionsAreDerivedFromScaleConstants() {
        final Barrel barrel = new Barrel(mock(ItemService.class));

        assertEquals(64 * 3, barrel.getWidth());
        assertEquals(64 * 3, barrel.getHeight());
        assertEquals(19 * 3, barrel.getCollissionWidth());
        assertEquals(21 * 3, barrel.getCollissionHeight());
        assertFalse(barrel.canRemove());
    }

    @Test
    void itemDropDropPositionIsConfigured() {
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
