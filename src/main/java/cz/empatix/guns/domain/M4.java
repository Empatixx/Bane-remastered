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
    private float lastTargetX;
    private float lastTargetY;
    private float lastShooterX;
    private float lastShooterY;

    public M4() {
        this.minDamage = 2;
        this.maxDamage = 2;

        this.inaccuracy = 0.f;
        this.fireRateTime = 750;
        this.reloadTime = 1300;

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
        if (bulletsToShoot > 0) {
            if (System.currentTimeMillis() - lastShootTime > BURST_BULLET_DELAY) {
                if(currentClip <= 0){
                    bulletsToShoot = 0;
                    return false;
                }
                lastShootTime = System.currentTimeMillis();
                LOGGER.info("Shooting from pistol clip: {}, ammo: {}", currentClip, ammo);
                final int damage = minDamage + (int) (ThreadLocalRandom.current().nextFloat() * (maxDamage - minDamage));
                final Bullet bullet = new Bullet();
                bullet.setPosition(shooterX, shooterY);
                bullet.setDirection(this.lastShooterX, this.lastShooterY, this.lastTargetX, this.lastTargetY, inaccuracy);
                bullets.add(bullet);
                bulletsToShoot--;
                currentClip--;
                return true;
            }
            return false;
        }
        if(!shooting){
            return false;
        }
        if(System.currentTimeMillis() - lastShootTime < fireRateTime){
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

            lastTargetX = targetX;
            lastTargetY = targetY;
            lastShooterX = shooterX;
            lastShooterY = shooterY;
            return true;
        }
        return false;
    }

    @Override
    public void reload() {
        if(reloading){
            return;
        }
        if(bulletsToShoot > 0){
            return;
        }
        if (ammo > 0 && currentClip < clipSize) {
            LOGGER.info("Reloading pistol clip: {}, ammo: {}", currentClip, ammo);
            reloading = true;
            lastReloadTime = System.currentTimeMillis();
        }
    }

    @Override
    public void update(final double dt) {
        bullets.forEach(b -> b.update(dt));
        if(reloading){
            if(System.currentTimeMillis() - lastReloadTime > reloadTime){
                reloading = false;
                reloadAmmo();
            }
        }
    }


    @Override
    public List<Renderable> toRender() {
        return new ArrayList<>(bullets);
    }

    @Override
    public String getAmmoCounter() {
        if(reloading){
            return "R";
        }
        return currentClip + "/" + ammo;
    }
}
