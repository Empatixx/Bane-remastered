package cz.empatix.guns.domain;

import cz.empatix.guns.ammo.Bullet;
import cz.empatix.render.attributes.Renderable;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class Shotgun extends AbstractGun implements Gun {
    private final static int BULLETS_PER_SHOT = 4;
    private final static Logger LOGGER = LoggerFactory.getLogger(Shotgun.class);
    private final List<Bullet> bullets;

    public Shotgun() {
        this.minDamage = 2;
        this.maxDamage = 3;

        this.inaccuracy = 0.15f;
        this.fireRate = 0.5f;

        this.clipSize = 6;
        this.currentClip = clipSize;
        this.maxAmmo = 48;
        this.ammo = maxAmmo;

        this.gunType = GunType.SHOTGUN;
        this.shooting = false;

        this.bullets = new ArrayList<>(clipSize);
    }

    @Override
    public void setShooting(final boolean shooting) {
        this.shooting = shooting;
    }

    @Override
    public boolean shoot(float shooterX, float shooterY, float targetX, float targetY
    ) {
        if(!shooting){
            return false;
        }
        if (currentClip > 0) {
            LOGGER.info("Shooting from pistol clip: {}, ammo: {}", currentClip, ammo);
            currentClip--;
            for(int i = 0;i<BULLETS_PER_SHOT;i++){
                final float inaccuracy = this.inaccuracy * (float) Math.pow(-1, i) * (i + 1) / BULLETS_PER_SHOT;
                final int damage = minDamage + (int) (ThreadLocalRandom.current().nextFloat() * (maxDamage - minDamage));
                final Bullet bullet = new Bullet();
                bullet.setPosition(shooterX, shooterY);
                bullet.setDirection(shooterX, shooterY, targetX, targetY, inaccuracy);
                bullets.add(bullet);
            }
            shooting = false;
            return true;
        }
        return false;
    }

    @Override
    public void reload() {
        if (ammo > 0 && currentClip < clipSize) {
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

    @Override
    public void update(final double dt) {
        bullets.forEach(b -> b.update(dt));
    }

    @Override
    public List<Renderable> toRender() {
        return new ArrayList<>(bullets);
    }

    @Override
    public String getAmmoCounter() {
        return currentClip + "/" + ammo;
    }
}
