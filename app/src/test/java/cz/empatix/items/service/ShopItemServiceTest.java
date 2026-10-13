package cz.empatix.items.service;

import cz.empatix.audio.init.ALInit;
import cz.empatix.items.domain.ShopItem;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.Mockito.verify;

/** Covers {@link ShopItemService} discounting a limited subset of items. */
class ShopItemServiceTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false; // ShopItem touches AudioService statics at class-load
    }

    @Test
    void discountAppliesToAtMostTheRequestedNumberOfItems() {
        final ShopItemService service = new ShopItemService();
        final List<ShopItem> items = new ArrayList<>();
        for (int i = 0; i < 5; i++) {
            final ShopItem item = Mockito.mock(ShopItem.class);
            items.add(item);
            service.add(item);
        }

        service.discount(0.5, 2);

        // Exactly 2 of the 5 mocks should have received discount(0.5); the shuffle picks which.
        int discounted = 0;
        for (final ShopItem item : items) {
            try {
                verify(item).discount(0.5);
                discounted++;
            } catch (final AssertionError ignored) {
                // not discounted
            }
        }
        assertEquals(2, discounted);
    }

    @Test
    void discountingMoreThanAvailableDiscountsAll() {
        final ShopItemService service = new ShopItemService();
        final ShopItem a = Mockito.mock(ShopItem.class);
        final ShopItem b = Mockito.mock(ShopItem.class);
        service.add(a);
        service.add(b);

        service.discount(0.25, 10); // request exceeds available

        verify(a).discount(0.25);
        verify(b).discount(0.25);
    }
}
