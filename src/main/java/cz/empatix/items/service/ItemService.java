package cz.empatix.items.service;

import cz.empatix.items.domain.ItemDrop;
import cz.empatix.render.attributes.Drawable;

import java.util.ArrayList;
import java.util.List;

public class ItemService {
    private final List<ItemDrop> itemDrops = new ArrayList<>(10);
    public void update(final float dt){
        itemDrops.forEach(itemDrop -> itemDrop.update(dt));
        itemDrops.removeIf(ItemDrop::canRemove);
    }
    public void add(final ItemDrop itemDrop){
        itemDrops.add(itemDrop);
    }
    public List<Drawable> toRender(){
        return List.of();
    }
}
