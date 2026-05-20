package cz.empatix.enemies.domain;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.TYPE)
public @interface SpawnableEnemy {
    EnemyType type() default EnemyType.COMMON;
    int minFloor() default 0;
    int maxFloor() default Integer.MAX_VALUE;

}
