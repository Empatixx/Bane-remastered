package cz.empatix.collision;


public interface DamageSource {
    int damage();
    DamageSourceType type();
    default void postDamage(final int damage){

    }
}
