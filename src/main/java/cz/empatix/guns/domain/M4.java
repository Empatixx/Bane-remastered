package cz.empatix.guns.domain;

import cz.empatix.guns.ammo.Bullet;
import cz.empatix.render.attributes.Renderable;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class M4 extends AbstractGun implements Gun{
    private final static Logger LOGGER = LoggerFactory.getLogger(Pistol.class);
    private final static int BULLETS_PER_SHOT = 4;
    private final static int BURST_BULLET_DELAY = 50;
    private final List<Bullet> bullets;
    private int bulletsToShoot;
    private long lastShootTime;
    public M4() {
        this.minDamage = 2;
        this.maxDamage = 2;

        this.inaccuracy = 0.f;
        this.fireRate = 750;

        this.clipSize = 32;
        this.currentClip = clipSize;
        this.maxAmmo = 250;
        this.ammo = maxAmmo;

        this.gunType = GunType.PISTOL;
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
        if (bulletsToShoot > 0) {
            if (System.currentTimeMillis() - lastShootTime > BURST_BULLET_DELAY) {
                lastShootTime = System.currentTimeMillis();
                LOGGER.info("Shooting from pistol clip: {}, ammo: {}", currentClip, ammo);
                final int damage = minDamage + (int) (ThreadLocalRandom.current().nextFloat() * (maxDamage - minDamage));
                final Bullet bullet = new Bullet();
                bullet.setPosition(shooterX, shooterY);
                bullet.setDirection(shooterX, shooterY, targetX, targetY, inaccuracy);
                bullets.add(bullet);
                bulletsToShoot--;
                currentClip--;
                return true;
            }
            return false;
        }
        if(System.currentTimeMillis() - lastShootTime < fireRate){
            return false;
        }
        if (currentClip > 0) {
            LOGGER.info("Shooting from pistol clip: {}, ammo: {}", currentClip, ammo);
            final int damage = minDamage + (int) (ThreadLocalRandom.current().nextFloat() * (maxDamage - minDamage));
            float inaccuracy = this.inaccuracy * (ThreadLocalRandom.current().nextFloat() - 0.5f);
            final Bullet bullet = new Bullet();
            bullet.setPosition(shooterX, shooterY);
            bullet.setDirection(shooterX, shooterY, targetX, targetY, inaccuracy);
            bullets.add(bullet);
            currentClip--;
            bulletsToShoot = BULLETS_PER_SHOT - 1;
            lastShootTime = System.currentTimeMillis();
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
