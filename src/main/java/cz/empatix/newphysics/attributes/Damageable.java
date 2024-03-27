package cz.empatix.newphysics.attributes;


public interface Damageable {
    int hit(final DamageSourceType source, final int damage);
}
