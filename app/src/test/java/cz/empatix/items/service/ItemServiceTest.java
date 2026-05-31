package cz.empatix.items.service;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.domain.DropSource;
import cz.empatix.items.domain.ItemDrop;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/** Covers the collection bookkeeping in {@link ItemService}. */
class ItemServiceTest {

    @Test
    void updateForwardsDtThenRemovesAndDestroysRemovableDrops() {
        final StatisticsService stats = mock(StatisticsService.class);
        final ItemService service = new ItemService(stats);

        final ItemDrop keep = mock(ItemDrop.class);
        when(keep.canRemove()).thenReturn(false);
        final ItemDrop remove = mock(ItemDrop.class);
        when(remove.canRemove()).thenReturn(true);
        when(remove.isPickedUp()).thenReturn(true);
        when(remove.getSource()).thenReturn(DropSource.NORMAL);

        service.add(keep);
        service.add(remove);

        service.update(0.016f);

        verify(keep).update(0.016f);
        verify(remove).update(0.016f);
        verify(remove).destroy();
        verify(keep, never()).destroy();
        // Picked-up NORMAL drop counts as a collected item.
        verify(stats).addItemCollected();

        assertEquals(1, service.getItems().size());
        assertSame(keep, service.getItems().get(0));
    }

    @Test
    void playerSourcedPickupIsNotCountedAsCollected() {
        final StatisticsService stats = mock(StatisticsService.class);
        final ItemService service = new ItemService(stats);

        final ItemDrop playerDrop = mock(ItemDrop.class);
        when(playerDrop.canRemove()).thenReturn(true);
        when(playerDrop.isPickedUp()).thenReturn(true);
        when(playerDrop.getSource()).thenReturn(DropSource.PLAYER);

        service.add(playerDrop);
        service.update(0f);

        verify(playerDrop).destroy();
        verify(stats, never()).addItemCollected();
        assertTrue(service.getItems().isEmpty());
    }

    @Test
    void despawnedButNotPickedUpDropIsRemovedWithoutStatistic() {
        final StatisticsService stats = mock(StatisticsService.class);
        final ItemService service = new ItemService(stats);

        final ItemDrop despawned = mock(ItemDrop.class);
        when(despawned.canRemove()).thenReturn(true);
        when(despawned.isPickedUp()).thenReturn(false);

        service.add(despawned);
        service.update(0f);

        verify(despawned).destroy();
        verify(stats, never()).addItemCollected();
        assertTrue(service.getItems().isEmpty());
    }

    @Test
    void cleanUpDestroysEverythingAndEmptiesTheList() {
        final ItemService service = new ItemService(mock(StatisticsService.class));
        final ItemDrop a = mock(ItemDrop.class);
        final ItemDrop b = mock(ItemDrop.class);
        service.add(a);
        service.add(b);

        service.cleanUp();

        verify(a).destroy();
        verify(b).destroy();
        assertTrue(service.getItems().isEmpty());
        assertTrue(service.toRender().isEmpty());
    }

    @Test
    void toRenderReflectsCurrentDropsAndFactoryRoundTrips() {
        final ItemService service = new ItemService(mock(StatisticsService.class));
        final ItemDrop a = mock(ItemDrop.class);
        service.add(a);

        assertEquals(1, service.toRender().size());

        final ItemFactory factory = mock(ItemFactory.class);
        service.setFactory(factory);
        assertSame(factory, service.getItemFactory());

        verify(a, times(0)).destroy();
    }
}
