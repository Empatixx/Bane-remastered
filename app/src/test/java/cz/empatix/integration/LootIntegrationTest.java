package cz.empatix.integration;

import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.guns.service.GunService;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.player.CoinService;
import cz.empatix.player.Player;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;

/**
 * Headless INTEGRATION test of the loot pipeline: a real {@link ItemFactory} mints a real coin drop
 * (auto-registering it with the real {@link InteractionService}); the drop is then located by a
 * proximity {@link InteractionService#query}, picked up (crediting the real {@link CoinService}), and
 * reaped by the real {@link ItemService} lifecycle. Wires factory + interaction + item + currency +
 * service-lifecycle together with real objects — the chain no single unit test covers.
 */
@Tag("integration")
@ExtendWith(HeadlessAssets.class)
class LootIntegrationTest {

    @Test
    void coinDropIsFoundByProximityPickedUpAndThenReaped() {
        final CoinService coinService = new CoinService();
        final ItemService itemService = new ItemService(mock(StatisticsService.class));
        final ItemFactory factory = new ItemFactory(
                mock(GunService.class), coinService, mock(Player.class), mock(ArtefactsService.class), new RngService(0L));

        final ItemDrop coin = factory.createCoinDrop(); // also registers with InteractionService
        coin.setPosition(100, 100);
        itemService.add(coin);

        // Proximity wiring: a query box overlapping the coin's footprint returns it.
        final var nearby = InteractionService.getInstance().query(new Vector2f(95, 95), new Vector2i(20, 20));
        assertTrue(nearby.contains(coin), "the dropped coin is discoverable by proximity");

        assertFalse(coinService.hasEnoughCoins(1), "no coins collected yet");
        coin.interact(Set.of()); // coins auto-collect on interaction (key-agnostic)
        assertTrue(coinService.hasEnoughCoins(1), "pickup credits the coin balance");

        assertEquals(1, itemService.getItems().size());
        itemService.update(0.016f); // a picked-up drop reports canRemove -> service reaps it
        assertTrue(itemService.getItems().isEmpty(), "the collected coin is removed from the world");
    }
}
