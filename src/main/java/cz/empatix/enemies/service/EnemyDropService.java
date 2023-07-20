package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;

import java.util.concurrent.ThreadLocalRandom;

public class EnemyDropService {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final static int GARAUNTEED_DROP = 1;
    private final ItemService itemService;
    private final ItemFactory itemFactory;
    private static int enemiesKilledWithoutDrops = 0;

    public EnemyDropService(final ItemService itemService){
        this.itemService = itemService;
        this.itemFactory = itemService.getItemFactory();
    }
    public void tryDrop(final Enemy e){
        enemiesKilledWithoutDrops++;
        if(enemiesKilledWithoutDrops + RANDOM.nextInt(6) > GARAUNTEED_DROP){
            final ItemDrop item = itemFactory.createItemDrop();
            item.setPosition(e.getX() + e.getWidth()/2f - item.getWidth()/2f, e.getY() + e.getHeight()/2f - item.getHeight()/2f);
            itemService.add(item);
            enemiesKilledWithoutDrops = 0;
        }
    }
}
