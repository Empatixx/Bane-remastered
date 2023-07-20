package cz.empatix.items.service;

public class ItemDespawnService {
    private final static long ITEM_DESPAWN_TIME = 10000L;
    private final long spawnTime;
    public ItemDespawnService(){
        spawnTime = System.currentTimeMillis();
    }
    public boolean canDespawn(){
        return System.currentTimeMillis() - spawnTime > ITEM_DESPAWN_TIME;
    }
    public boolean canBlink(){
        return System.currentTimeMillis() - spawnTime > ITEM_DESPAWN_TIME - 2000L;
    }
}
