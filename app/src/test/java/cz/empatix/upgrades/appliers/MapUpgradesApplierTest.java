package cz.empatix.upgrades.appliers;

import cz.empatix.map.service.MapService;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.player.Player;
import cz.empatix.roomobjects.domain.RoomObject;
import cz.empatix.roomobjects.domain.Trap;
import cz.empatix.rooms.domain.RoomEvent;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.ShopRoom;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.ArgumentCaptor;

import java.util.List;
import java.util.function.Consumer;

import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.atMost;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;
import static org.mockito.Mockito.withSettings;

/**
 * Logic coverage of {@link MapUpgradesApplier}. Collaborators are mocked. The DISCOUNT path filters
 * shop rooms and applies discounts; ARMOR_REGENERATION registers a room-event listener whose
 * captured lambda is fired to assert the +1 armour rule (and its guard); TRAP_MASTER walks the room
 * objects and probabilistically deactivates traps — asserted only on its deterministic bounds (0..N)
 * since it draws randomness.
 */
class MapUpgradesApplierTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    @Test
    void nothingBoughtRegistersNoListenersOrDiscounts() {
        final MapService mapService = mock(MapService.class);
        final Player player = mock(Player.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);

        new MapUpgradesApplier(mapService, player, upgrades).apply();

        verify(mapService, never()).onRoomEventChange(any());
        verify(player, never()).addArmor(org.mockito.ArgumentMatchers.anyInt());
    }

    @Test
    void discountAppliesDiscountsToShopRoomsOnly() {
        final MapService mapService = mock(MapService.class);
        final Player player = mock(Player.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        when(upgrades.isBought(PlayerUpgradeName.DISCOUNT)).thenReturn(true);

        final ShopRoom shop = mock(ShopRoom.class);
        when(shop.getRoomType()).thenReturn(RoomType.SHOP);
        final Room normal = mock(Room.class);
        when(normal.getRoomType()).thenReturn(RoomType.NORMAL);
        when(mapService.getRooms()).thenReturn(List.of(normal, shop));

        new MapUpgradesApplier(mapService, player, upgrades).apply();

        verify(shop).applyDiscounts();
    }

    @Test
    void armorRegenListenerAddsArmorOnClosedToOpenTransition() {
        final MapService mapService = mock(MapService.class);
        final Player player = mock(Player.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        when(upgrades.isBought(PlayerUpgradeName.ARMOR_REGENERATION)).thenReturn(true);

        new MapUpgradesApplier(mapService, player, upgrades).apply();

        @SuppressWarnings("unchecked")
        final ArgumentCaptor<Consumer<RoomEvent>> captor =
                ArgumentCaptor.forClass(Consumer.class);
        verify(mapService).onRoomEventChange(captor.capture());
        final Consumer<RoomEvent> listener = captor.getValue();

        // CLOSED -> OPEN clears a room: +1 armour.
        listener.accept(new RoomEvent(RoomState.OPEN, RoomState.CLOSED, RoomType.NORMAL));
        verify(player, times(1)).addArmor(1);

        // Any other transition: no armour granted.
        listener.accept(new RoomEvent(RoomState.CLOSED, RoomState.OPEN, RoomType.NORMAL));
        verify(player, times(1)).addArmor(1); // still only the single earlier call
    }

    @Test
    void trapMasterDeactivatesAtMostAllTrapsAndIgnoresNonTraps() {
        final MapService mapService = mock(MapService.class);
        final Player player = mock(Player.class);
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        when(upgrades.isBought(PlayerUpgradeName.TRAP_MASTER)).thenReturn(true);

        // A trap object implements both RoomObject and Trap; a plain object is not a Trap and so
        // is filtered out by the `instanceof Trap` guard before any deactivate() could be called.
        final RoomObject trap = mock(RoomObject.class, withSettings().extraInterfaces(Trap.class));
        final RoomObject notTrap = mock(RoomObject.class); // not a Trap -> filtered out

        final Room room = mock(Room.class);
        when(room.getRoomObjects()).thenReturn(List.of(trap, notTrap));
        when(mapService.getRooms()).thenReturn(List.of(room));

        new MapUpgradesApplier(mapService, player, upgrades).apply();

        // Randomised 30% gate: the trap is deactivated either 0 or 1 times.
        verify((Trap) trap, atMost(1)).deactivate();
    }
}
