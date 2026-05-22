package cz.empatix.guns.domain;

import cz.empatix.audio.service.AudioService;
import cz.empatix.sound.SoundEffect;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public abstract class AbstractGun {
    private final static Logger LOGGER = LoggerFactory.getLogger(AbstractGun.class);
    protected final AudioService audioService;
    protected int minDamage;
    protected  int maxDamage;

    protected float inaccuracy;
    protected int reloadTime;
    protected int fireRateTime;

    protected int clipSize;
    protected int currentClip;
    protected int ammo;
    protected int maxAmmo;

    protected GunType gunType;

    protected boolean shooting;
    protected boolean reloading;

    protected long lastShootTime;
    protected long lastReloadTime;

    protected AbstractGun(final AudioService audioService) {
        this.audioService = audioService;
    }

    /** The sound played when this gun starts reloading. */
    protected abstract SoundEffect reloadSound();

    /** Shared reload trigger: starts a reload if not already reloading and a top-up is possible. */
    public void reload() {
        if (reloading) {
            return;
        }
        if (ammo > 0 && currentClip < clipSize) {
            LOGGER.info("Reloading clip: {}, ammo: {}", currentClip, ammo);
            reloading = true;
            lastReloadTime = System.currentTimeMillis();
            audioService.play(reloadSound());
        }
    }

    /** Shared reload completion: finishes the reload and tops up the clip once the timer elapses. */
    protected void updateReload() {
        if (reloading && System.currentTimeMillis() - lastReloadTime > reloadTime) {
            reloading = false;
            reloadAmmo();
        }
    }

    protected void reloadAmmo(){
        final int ammoToReload = clipSize - currentClip;
        if (ammoToReload <= ammo) {
            currentClip += ammoToReload;
            ammo -= ammoToReload;
        } else {
            currentClip += ammo;
            ammo = 0;
        }
    }
}
