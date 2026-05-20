package cz.empatix.enemies.domain;

import java.util.function.Supplier;

public record EnemySupplier(
        Supplier<Enemy> supplier,
        int minFloor,
        int maxFloor
) {
    public boolean isSpawnable(final int floor) {
        return floor >= minFloor && floor <= maxFloor;
    }
}
