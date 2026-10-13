package cz.empatix.player;

import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for {@link CoinService}'s coin balance arithmetic and the
 * {@code hasEnoughCoins} affordability check. The HUD sub-objects ({@code CommonImage}/{@code CommonText})
 * are built against the GL-free fakes from {@link MockedAssets}; their render paths are never invoked.
 */
@ExtendWith(HeadlessAssets.class)
class CoinServiceTest {

    @Test
    void freshServiceHasNoCoinsAndCannotAffordAnything() {
        final CoinService coinService = new CoinService();

        assertTrue(coinService.hasEnoughCoins(0), "zero is always affordable");
        assertFalse(coinService.hasEnoughCoins(1));
    }

    @Test
    void addingCoinsIncreasesTheBalance() {
        final CoinService coinService = new CoinService();

        coinService.addCoins(7);

        assertTrue(coinService.hasEnoughCoins(7));
        assertFalse(coinService.hasEnoughCoins(8));
    }

    @Test
    void addsAccumulateAcrossCalls() {
        final CoinService coinService = new CoinService();

        coinService.addCoins(3);
        coinService.addCoins(4);

        assertTrue(coinService.hasEnoughCoins(7));
    }

    @Test
    void removingCoinsLowersTheBalance() {
        final CoinService coinService = new CoinService();
        coinService.addCoins(10);

        coinService.removeCoins(4);

        assertTrue(coinService.hasEnoughCoins(6));
        assertFalse(coinService.hasEnoughCoins(7));
    }

    @Test
    void addingANonPositiveAmountIsRejected() {
        final CoinService coinService = new CoinService();

        assertThrows(IllegalArgumentException.class, () -> coinService.addCoins(0));
        assertThrows(IllegalArgumentException.class, () -> coinService.addCoins(-5));
    }

    @Test
    void removingANonPositiveAmountIsRejected() {
        final CoinService coinService = new CoinService();

        assertThrows(IllegalArgumentException.class, () -> coinService.removeCoins(0));
        assertThrows(IllegalArgumentException.class, () -> coinService.removeCoins(-1));
    }
}
