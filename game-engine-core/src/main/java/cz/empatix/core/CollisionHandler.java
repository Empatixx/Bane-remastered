package cz.empatix.core;

/**
 * Cross-cutting reaction to a physics collision. The engine invokes registered
 * handlers when two bodies collide, passing each body's user data (the game-side
 * entity). The engine itself has no knowledge of what the handler does.
 */
public interface CollisionHandler {
    /**
     * @param a user data of the first colliding body (may be {@code null})
     * @param b user data of the second colliding body (may be {@code null})
     */
    void onCollision(Object a, Object b);
}
