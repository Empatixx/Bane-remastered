package cz.empatix.items.service;

public class ItemDespawnService {
    public final static long DEFAULT_ITEM_DESPAWN_TIME = 10000L;
    private long despawnTime;
    private final long spawnTime;
    public ItemDespawnService(){
        spawnTime = System.currentTimeMillis();
        despawnTime = DEFAULT_ITEM_DESPAWN_TIME;
    }
    public boolean canDespawn(){
        return System.currentTimeMillis() - spawnTime > despawnTime;
    }
    public boolean canBlink(){
        return System.currentTimeMillis() - spawnTime > despawnTime - 2000L;
    }
    public void setDespawnTime(final long despawnTime){
        this.despawnTime = despawnTime;
    }
}
