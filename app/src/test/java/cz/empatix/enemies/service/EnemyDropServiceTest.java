package cz.empatix.enemies.service;

import cz.empatix.common.RngService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import org.junit.jupiter.api.Test;

import java.lang.reflect.Field;

import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Verifies the guaranteed-drop bookkeeping of {@link EnemyDropService}. The drop decision combines a
 * static "kills without drops" counter with a random roll; the counter is seeded via reflection so
 * the deterministic branches (always-drop / never-drop) can be asserted without relying on the dice.
 */
class EnemyDropServiceTest {

    private static void setCounter(final int value) throws Exception {
        final Field field = EnemyDropService.class.getDeclaredField("enemiesKilledWithoutDrops");
        field.setAccessible(true);
        field.set(null, value);
    }

    private static int getCounter() throws Exception {
        final Field field = EnemyDropService.class.getDeclaredField("enemiesKilledWithoutDrops");
        field.setAccessible(true);
        return (int) field.get(null);
    }

    private static EnemyDropService newService(final ItemFactory factory, final ItemService itemService) {
        when(itemService.getItemFactory()).thenReturn(factory);
        return new EnemyDropService(itemService, new RngService(0L));
    }

    @Test
    void dropsAndResetsCounterWhenThresholdGuaranteed() throws Exception {
        final ItemFactory factory = mock(ItemFactory.class);
        final ItemService itemService = mock(ItemService.class);
        final EnemyDropService service = newService(factory, itemService);

        final ItemDrop drop = mock(ItemDrop.class);
        when(factory.createItemDrop()).thenReturn(drop);
        final Enemy enemy = mock(Enemy.class);

        // counter starts large -> counter + roll(0..5) is always > GUARANTEED_DROP(4)
        setCounter(100);
        service.tryDrop(enemy);

        verify(factory).createItemDrop();
        verify(itemService).add(drop);
        verify(drop).setPosition(anyFloat(), anyFloat());
        org.junit.jupiter.api.Assertions.assertEquals(0, getCounter(), "drop resets the streak counter");
    }

    @Test
    void doesNotDropWhenStreakFarBelowThreshold() throws Exception {
        final ItemFactory factory = mock(ItemFactory.class);
        final ItemService itemService = mock(ItemService.class);
        final EnemyDropService service = newService(factory, itemService);
        final Enemy enemy = mock(Enemy.class);

        // counter very negative -> counter + roll(0..5) can never exceed 4
        setCounter(-100);
        service.tryDrop(enemy);

        verify(factory, never()).createItemDrop();
        verify(itemService, never()).add(any());
        org.junit.jupiter.api.Assertions.assertEquals(-99, getCounter(), "no-drop still increments the streak");
    }

    @Test
    void streakIncrementsOnEachNonDroppingKill() throws Exception {
        final ItemFactory factory = mock(ItemFactory.class);
        final ItemService itemService = mock(ItemService.class);
        final EnemyDropService service = newService(factory, itemService);
        final Enemy enemy = mock(Enemy.class);

        setCounter(-100);
        service.tryDrop(enemy);
        service.tryDrop(enemy);

        verify(factory, never()).createItemDrop();
        org.junit.jupiter.api.Assertions.assertEquals(-98, getCounter());
        verify(itemService, times(0)).add(any());
    }
}
