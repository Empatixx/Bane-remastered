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

    /** The sound played on a successful shot. */
    protected abstract SoundEffect shootSound();

    /** Spawn this gun's projectile(s) for one trigger pull. The clip has already been decremented. */
    protected abstract void spawnBullets(float shooterX, float shooterY, float targetX, float targetY);

    /**
     * Semi-auto guns fire once per trigger pull and clear {@code shooting} after each shot (or dry
     * click); full-auto guns keep firing while the trigger is held. Defaults to semi-auto.
     */
    protected boolean isSemiAuto() {
        return true;
    }

    /**
     * Shared fire control: enforces the trigger/reload/fire-rate gates, then either spawns this gun's
     * projectiles or plays the empty-clip click. Guns with bespoke firing (e.g. burst) override this.
     */
    public boolean shoot(final float shooterX, final float shooterY, final float targetX, final float targetY) {
        if (!shooting || reloading) {
            return false;
        }
        if (System.currentTimeMillis() - lastShootTime < fireRateTime) {
            return false;
        }
        if (currentClip > 0) {
            currentClip--;
            spawnBullets(shooterX, shooterY, targetX, targetY);
            lastShootTime = System.currentTimeMillis();
            if (isSemiAuto()) {
                shooting = false;
            }
            LOGGER.info("Shooting from {} clip: {}, ammo: {}", gunType, currentClip, ammo);
            audioService.play(shootSound());
            return true;
        }
        // Empty clip: semi-auto relies on the trigger reset to stop repeat clicks; full-auto throttles
        // the dry click through the fire-rate gate instead.
        if (isSemiAuto()) {
            shooting = false;
        } else {
            lastShootTime = System.currentTimeMillis();
        }
        audioService.play(SoundEffect.EMPTY_GUN_SHOOT);
        return false;
    }

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
