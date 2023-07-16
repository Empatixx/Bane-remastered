package cz.empatix.items.service;

import cz.empatix.items.domain.AmmoItem;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.domain.RecoverPotion;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Supplier;

public class ItemFactory {
    private final List<Supplier<ItemDrop>> itemDropSuppliers = List.of(AmmoItem::new, RecoverPotion::new);

    public ItemDrop createItemDrop() {
        return itemDropSuppliers.get(ThreadLocalRandom.current().nextInt(itemDropSuppliers.size())).get();
    }
}
