package cz.empatix.audio.domain;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public enum SoundEffect {
    BUY("buy.ogg"),
    COIN("coin.ogg"),
    DEATH("playerdeath.ogg"),
    LOW_HEALTH("lowhealth.ogg"),
    CLICK("menuclick.ogg"),
    PICKUP("pickup.ogg"),
    SWITCH_GUN("switchgun.ogg"),
    PLAYER_HURT("playerhurt_1.ogg", "playerhurt_2.ogg");
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final List<String> paths;
    SoundEffect(final String... path) {
        this.paths = List.of(path);
    }
    public String getPath(){
        return paths.get(RANDOM.nextInt(paths.size()));
    }
}
