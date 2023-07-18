package cz.empatix.items.service;

import cz.empatix.items.domain.ItemDrop;
import cz.empatix.render.attributes.Drawable;

import java.util.ArrayList;
import java.util.List;

public class ItemService {
    private final List<ItemDrop> itemDrops = new ArrayList<>(10);
    private ItemFactory itemFactory;
    public void update(final float dt){
        itemDrops.forEach(itemDrop -> itemDrop.update(dt));
        itemDrops.removeIf(ItemDrop::canRemove);
    }
    public void add(final ItemDrop itemDrop){
        itemDrops.add(itemDrop);
    }
    public List<Drawable> toRender(){
        return List.copyOf(itemDrops);
    }
    public void setFactory(final ItemFactory itemFactory) {
        this.itemFactory = itemFactory;
    }

    public ItemDrop randomDrop() {
        return itemFactory.createItemDrop();
    }
}
