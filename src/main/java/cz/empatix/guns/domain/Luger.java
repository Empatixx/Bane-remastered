package cz.empatix.guns.domain;

import cz.empatix.guns.ammo.Bullet;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.service.CameraShaker;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class Luger extends AbstractGun implements Gun{
    private final static Logger LOGGER = LoggerFactory.getLogger(Pistol.class);
    private final CameraShaker cameraShaker = new CameraShaker();
    private final List<Bullet> bullets;
    private int bulletsToShoot;

    public Luger() {
        this.minDamage = 1;
        this.maxDamage = 2;

        this.inaccuracy = 0.35f;
        this.fireRateTime = 250;
        this.reloadTime = 700;

        this.clipSize = 9;
        this.currentClip = clipSize;
        this.maxAmmo = 150;
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
            shooting = false;
            lastShootTime = System.currentTimeMillis();
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
            LOGGER.info("Reloading clip: {}, ammo: {}", currentClip, ammo);
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
        cameraShaker.update((float)dt);
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
