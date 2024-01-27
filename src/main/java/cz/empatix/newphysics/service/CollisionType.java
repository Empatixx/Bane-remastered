package cz.empatix.newphysics.service;

import java.util.EnumSet;
import java.util.List;
import java.util.Set;

public enum CollisionType {
    ITEM,
    TRAP,
    PLAYER,
    ENEMY,
    NONE,
    PLAYER_BULLET,
    ENEMY_BULLET;
    private Set<CollisionType> collidesWith;

    static {
        ITEM.collidesWith = EnumSet.noneOf(CollisionType.class);
        TRAP.collidesWith = EnumSet.noneOf(CollisionType.class);
        PLAYER.collidesWith =  EnumSet.of(ENEMY_BULLET);
        ENEMY.collidesWith =  EnumSet.of(PLAYER_BULLET);
        NONE.collidesWith =  EnumSet.noneOf(CollisionType.class);
        PLAYER_BULLET.collidesWith =  EnumSet.of(ENEMY);
        ENEMY_BULLET.collidesWith =  EnumSet.of(PLAYER);
    }

    public boolean collidesWith(final CollisionType type) {

        return collidesWith.contains(type);
    }
}