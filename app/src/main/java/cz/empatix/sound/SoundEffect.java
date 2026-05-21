package cz.empatix.sound;

import cz.empatix.audio.domain.SoundEffectClip;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public enum SoundEffect implements SoundEffectClip {
    BUY("buy.ogg"),
    COIN("coin.ogg"),
    DEATH("playerdeath.ogg"),
    LOW_HEALTH("lowhealth.ogg"),
    CLICK("menuclick.ogg"),
    PICKUP("pickup.ogg"),
    SWITCH_GUN("switchgun.ogg"),
    PLAYER_HURT("playerhurt_1.ogg", "playerhurt_2.ogg"),
    EMPTY_GUN_SHOOT("guns/empty_gun_shoot.ogg"),
    PISTOL_SHOOT("guns/pistol_shoot_1.ogg", "guns/pistol_shoot_2.ogg"),
    SHOTGUN_SHOOT("guns/shotgun_shoot.ogg"),
    M4_SHOOT("guns/M4_shoot.ogg"),
    REVOLVER_SHOOT("guns/revolver_shoot.ogg"),
    UZI_SHOOT("guns/uzi_shoot.ogg"),
    PISTOL_RELOAD("guns/pistol_reload.ogg"),
    SHOTGUN_RELOAD("guns/shotgun_reload.ogg"),
    TALENT_UNLOCK("talent_unlock.ogg"),
    M4_RELOAD("guns/M4_reload.ogg"),
    OPEN_CHEST("open_chest_1.ogg", "open_chest_2.ogg", "open_chest_3.ogg"),
    PLAYER_DASH("player_dash_1.ogg", "player_dash_2.ogg"),
    SHOW_MAP("map_1.ogg", "map_2.ogg"),
    HEAL("heal.ogg"),
    WALL_HIT("guns/wall_hit.ogg"),
    ENEMY_HIT("guns/enemy_hit.ogg"),
    EXPLOSION("explosion.ogg"),
    SACRIFICE("sacrifice.ogg"),
    SLOTMACHINE_WIN("slotmachine_win.ogg"),
    SLOTMACHINE_LOSE("slotmachine_lose.ogg"),
    SLOTMACHINE_SPIN("slotmachine_spin.ogg"),
    SLOTMACHINE_CASHOUT("slotmachine_cashout.ogg");
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final List<String> paths;
    SoundEffect(final String... path) {
        this.paths = List.of(path);
    }
    public String getPath(){
        return paths.get(RANDOM.nextInt(paths.size()));
    }
}
