package cz.empatix.map.domain;

import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;

public enum RoomDirection{
    TOP,
    BOTTOM,
    LEFT,
    RIGHT;
    public RoomDirection getOpposite(){
        return switch (this) {
            case TOP -> BOTTOM;
            case BOTTOM -> TOP;
            case LEFT -> RIGHT;
            case RIGHT -> LEFT;
        };
    }
    public static RoomDirection getRandom(){
        return RoomDirection.values()[ThreadLocalRandom.current().nextInt(0, RoomDirection.values().length)];
    }
    public static RoomDirection getRandomWithout(final RoomDirection ...directions){
        final Set<RoomDirection> directionsSet = Set.of(directions);
        RoomDirection random;
        do {
            random = getRandom();
        } while (directionsSet.contains(random));
        return random;
    }
    public boolean isHorizontal(){
        return this == LEFT || this == RIGHT;
    }
    public boolean isVertical(){
        return this == TOP || this == BOTTOM;
    }
}
