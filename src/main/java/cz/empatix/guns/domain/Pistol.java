package cz.empatix.guns.domain;

public class Pistol extends AbstractGun implements Gun{
    public Pistol() {
        this.minDamage = 1;
        this.maxDamage = 3;

        this.inaccuracy = 0.1f;
        this.fireRate = 0.5f;

        this.clipSize = 7;
        this.currentClip = clipSize;
        this.maxAmmo = 120;
        this.ammo = maxAmmo;

        this.gunType = GunType.PISTOL;
        this.shooting = false;
    }

    @Override
    public void setShooting(final boolean shooting) {
        this.shooting = shooting;
    }

    @Override
    public boolean shoot(float x, float y) {
        return false;
    }

    @Override
    public void reload() {

    }

    @Override
    public void update(final double dt) {

    }
}
