package cz.empatix.items.service;

import cz.empatix.items.domain.ShopItem;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.List;

public class ShopItemService {
    private final List<ShopItem> shopItems;
    public ShopItemService() {
        this.shopItems = new ArrayList<>();
    }
    public void discount(final double discountPercent,
                         final int totalItems){
        Collections.shuffle(shopItems);
        shopItems.stream()
                .limit(totalItems)
                .forEach(shopItem -> shopItem.discount(discountPercent));
    }

    public void add(final ShopItem shopItem) {
        shopItems.add(shopItem);
    }
}
