package cz.empatix.player;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for {@link CoinService}'s coin balance arithmetic and the
 * {@code hasEnoughCoins} affordability check. The HUD sub-objects ({@code CommonImage}/{@code CommonText})
 * are built against the GL-free fakes from {@link MockedAssets}; their render paths are never invoked.
 */
class CoinServiceTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @Test
    void freshServiceHasNoCoinsAndCannotAffordAnything() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CoinService coinService = new CoinService();

            assertTrue(coinService.hasEnoughCoins(0), "zero is always affordable");
            assertFalse(coinService.hasEnoughCoins(1));
        }
    }

    @Test
    void addingCoinsIncreasesTheBalance() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CoinService coinService = new CoinService();

            coinService.addCoins(7);

            assertTrue(coinService.hasEnoughCoins(7));
            assertFalse(coinService.hasEnoughCoins(8));
        }
    }

    @Test
    void addsAccumulateAcrossCalls() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CoinService coinService = new CoinService();

            coinService.addCoins(3);
            coinService.addCoins(4);

            assertTrue(coinService.hasEnoughCoins(7));
        }
    }

    @Test
    void removingCoinsLowersTheBalance() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CoinService coinService = new CoinService();
            coinService.addCoins(10);

            coinService.removeCoins(4);

            assertTrue(coinService.hasEnoughCoins(6));
            assertFalse(coinService.hasEnoughCoins(7));
        }
    }

    @Test
    void addingANonPositiveAmountIsRejected() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CoinService coinService = new CoinService();

            assertThrows(IllegalArgumentException.class, () -> coinService.addCoins(0));
            assertThrows(IllegalArgumentException.class, () -> coinService.addCoins(-5));
        }
    }

    @Test
    void removingANonPositiveAmountIsRejected() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CoinService coinService = new CoinService();

            assertThrows(IllegalArgumentException.class, () -> coinService.removeCoins(0));
            assertThrows(IllegalArgumentException.class, () -> coinService.removeCoins(-1));
        }
    }
}
