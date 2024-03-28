package cz.empatix.newphysics.attributes;


public interface DamageSource {
    int damage();
    DamageSourceType type();
    default void postDamage(final int damage){

    }
}
