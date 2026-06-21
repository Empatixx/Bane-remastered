package cz.empatix.enemies.service;

import cz.empatix.common.RngService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;

public class EnemyDropService {
    private final static int GUARANTEED_DROP = 4;
    private final ItemService itemService;
    private final ItemFactory itemFactory;
    private final RngService rng;
    private static int enemiesKilledWithoutDrops = 0;

    public EnemyDropService(final ItemService itemService, final RngService rng){
        this.itemService = itemService;
        this.itemFactory = itemService.getItemFactory();
        this.rng = rng;
    }
    public void tryDrop(final Enemy e){
        enemiesKilledWithoutDrops++;
        if(enemiesKilledWithoutDrops + rng.nextInt(6) > GUARANTEED_DROP){
            final ItemDrop item = itemFactory.createItemDrop();
            item.setPosition(e.getX() + e.getWidth()/2f - item.getWidth()/2f, e.getY() + e.getHeight()/2f - item.getHeight()/2f);
            itemService.add(item);
            enemiesKilledWithoutDrops = 0;
        }
    }
}
