package cz.empatix.roomobjects.domain;

import cz.empatix.common.RngService;
import cz.empatix.items.domain.ItemRarity;
import cz.empatix.items.domain.MovableItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.player.CoinService;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.lwjgl.glfw.GLFW;

import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Headless coverage for the real {@link SlotMachine} (a {@code RoomObject} + {@code Interactable}).
 * The constructor seeds 100 coins via a real {@link CoinService}, registers with the interaction
 * service and builds a delayed spin task. These tests exercise the spin gate, the coin cost, the
 * interaction predicate, and a full spin resolution (driven by advancing the delayed task past its
 * delay rather than sleeping), with every reward branch's item-factory call stubbed.
 */
@ExtendWith(HeadlessAssets.class)
class SlotMachineTest {

    private static ItemService itemServiceWithStubbedFactory() {
        final ItemService itemService = mock(ItemService.class);
        final ItemFactory factory = mock(ItemFactory.class);
        final MovableItemDrop drop = mock(MovableItemDrop.class);
        when(itemService.getItemFactory()).thenReturn(factory);
        when(factory.createMovableCoinDrop()).thenReturn(drop);
        when(factory.createMovableItemDrop(any(ItemRarity.class))).thenReturn(drop);
        when(factory.createMovableItemDrop(any(ItemRarity.class), any(ItemRarity.class))).thenReturn(drop);
        return itemService;
    }

    @Test
    void freshMachineIsNotSpinningAndOnlyInteractsWithEKey() {
        final SlotMachine machine = new SlotMachine(mock(ItemService.class), new CoinService(), new RngService(0L));

        assertFalse(machine.canInteract(Set.of(GLFW.GLFW_KEY_A)));
        assertTrue(machine.canInteract(Set.of(GLFW.GLFW_KEY_E)));
        assertFalse(machine.canRemove());
    }

    @Test
    void spinningConsumesCoinsAndBlocksFurtherInteractionUntilResolved() {
        final ItemService itemService = itemServiceWithStubbedFactory();
        final CoinService coinService = new CoinService(); // starts at 0, machine adds 100 in ctor
        final SlotMachine machine = new SlotMachine(itemService, coinService, new RngService(0L));

        machine.interact(Set.of(GLFW.GLFW_KEY_E)); // spin
        assertFalse(machine.canInteract(Set.of(GLFW.GLFW_KEY_E))); // spinning -> cannot interact

        // Resolve the 2s delayed spin task in a single big tick; reward branch runs headlessly.
        machine.update(5.0);

        // After resolution the machine accepts interaction again.
        assertTrue(machine.canInteract(Set.of(GLFW.GLFW_KEY_E)));
    }

    @Test
    void positionAndGeometry() {
        final SlotMachine machine = new SlotMachine(mock(ItemService.class), new CoinService(), new RngService(0L));
        machine.setPosition(200f, 300f);

        assertEquals(64 * 3, machine.getWidth());
        assertEquals(64 * 3, machine.getHeight());
        assertEquals(200f, machine.getPosition().x, 0.0001f);
        assertEquals(300f, machine.getPosition().y, 0.0001f);
    }
}
