package cz.empatix.guns.domain;

public abstract class AbstractGun {
    protected int minDamage;
    protected  int maxDamage;

    protected float inaccuracy;
    protected float fireRate;

    protected int clipSize;
    protected int currentClip;
    protected int ammo;
    protected int maxAmmo;

    protected GunType gunType;

    protected boolean shooting;
}
