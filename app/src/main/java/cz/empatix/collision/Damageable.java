package cz.empatix.collision;


public interface Damageable {
    int hit(final DamageSourceType source, final int damage);
}
