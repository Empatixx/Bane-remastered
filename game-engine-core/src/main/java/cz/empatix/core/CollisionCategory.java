package cz.empatix.core;

/**
 * Opaque collision category attached to a physics body. The engine only asks
 * whether two categories collide; the concrete categories and the collision
 * matrix are defined by the game.
 */
public interface CollisionCategory {
    boolean collidesWith(CollisionCategory other);
}
