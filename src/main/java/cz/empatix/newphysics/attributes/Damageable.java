package cz.empatix.newphysics.attributes;


public interface Damageable {
    void hit(final DamageSourceType source, final int damage);
}
