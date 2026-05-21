package cz.empatix.collision;

import cz.empatix.core.CollisionCategory;

import java.util.EnumSet;
import java.util.Set;

public enum CollisionType implements CollisionCategory {
    ITEM,
    TRAP,
    PLAYER,
    ENEMY,
    NONE,
    PLAYER_BULLET,
    ENEMY_BULLET,
    ROOM_OBJECT,
    TILE;
    private Set<CollisionType> collidesWith;

    static {
        ITEM.collidesWith = EnumSet.of(TILE);
        TRAP.collidesWith = EnumSet.of(TILE);
        PLAYER.collidesWith =  EnumSet.of(ENEMY_BULLET, ROOM_OBJECT, TILE);
        ENEMY.collidesWith =  EnumSet.of(PLAYER_BULLET, TILE);
        NONE.collidesWith =  EnumSet.noneOf(CollisionType.class);
        PLAYER_BULLET.collidesWith =  EnumSet.of(ENEMY, ROOM_OBJECT, TILE);
        ENEMY_BULLET.collidesWith =  EnumSet.of(PLAYER, ROOM_OBJECT, TILE);
        ROOM_OBJECT.collidesWith =  EnumSet.of(PLAYER, PLAYER_BULLET, ENEMY_BULLET, ROOM_OBJECT, TILE);
        TILE.collidesWith = EnumSet.of(PLAYER, ENEMY, PLAYER_BULLET, ENEMY_BULLET, ROOM_OBJECT);
    }

    @Override
    public boolean collidesWith(final CollisionCategory type) {
        return collidesWith.contains(type);
    }
}
