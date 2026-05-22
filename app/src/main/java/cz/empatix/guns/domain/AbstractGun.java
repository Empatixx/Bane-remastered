package cz.empatix.guns.domain;

import cz.empatix.audio.service.AudioService;

public abstract class AbstractGun {
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
