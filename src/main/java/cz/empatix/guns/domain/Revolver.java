package cz.empatix.guns.domain;

import cz.empatix.guns.ammo.Bullet;
import cz.empatix.render.attributes.Renderable;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class Revolver extends AbstractGun implements Gun {
    private final static Logger LOGGER = LoggerFactory.getLogger(Revolver.class);
    private final List<Bullet> bullets;
    public Revolver() {
        this.minDamage = 4;
        this.maxDamage = 6;

        this.inaccuracy = 0.2f;
        this.fireRate = 0.5f;

        this.clipSize = 6;
        this.currentClip = clipSize;
        this.maxAmmo = 60;
        this.ammo = maxAmmo;

        this.gunType = GunType.PISTOL;
        this.shooting = false;

        this.bullets = new ArrayList<>(clipSize);
    }

    @Override
    public void setShooting(boolean shooting) {

    }

    @Override
    public boolean shoot(float shooterX, float shooterY, float targetX, float targetY) {
        if(!shooting){
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
    public void update(double dt) {
        bullets.forEach(b -> b.update(dt));

    }

    @Override
    public List<Renderable> toRender() {
        return new ArrayList<>(bullets);
    }

    @Override
    public String getAmmoCounter() {
        return null;
    }
}
