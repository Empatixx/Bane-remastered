package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.*;
import cz.empatix.enemies.domain.bosses.Golem;
import cz.empatix.enemies.domain.bosses.KingSlime;
import org.reflections.Reflections;

import java.lang.reflect.InvocationTargetException;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Supplier;

public class EnemyFactory {
    private final List<Supplier<Enemy>> commonEnemiesSuppliers = List.of(Bat::new, Slime::new, DemonEye::new, Ghost::new, RedSlime::new, Snake::new, Rat::new,
            EyeWatcher::new, Spider::new);
    private final List<Supplier<Enemy>> bossEnemiesSuppliers = List.of(Golem::new);//KingSlime::new
    public EnemyFactory(){
        final Reflections reflections = new Reflections("cz.empatix.enemies.domain");
        final Set<Class<?>> annotatedClasses = reflections.getTypesAnnotatedWith(SpawnableEnemy.class);
        final List<Supplier<Enemy>> commonEnemiesSuppliers = new ArrayList<>();
        final List<Supplier<Enemy>> bossEnemiesSuppliers = new ArrayList<>();

        for (Class<?> annotatedClass : annotatedClasses) {
            final SpawnableEnemy annotation = annotatedClass.getAnnotation(SpawnableEnemy.class);
            try {
                final Supplier<Enemy> supplier = () -> {
                    try {
                        return (Enemy) annotatedClass.getConstructor().newInstance();
                    } catch (Exception e) {
                        throw new RuntimeException(e);
                    }
                };
                switch (annotation.type()) {
                    case COMMON -> commonEnemiesSuppliers.add(supplier);
                    case BOSS -> bossEnemiesSuppliers.add(supplier);
                }
            } catch (Exception e) {
               throw new RuntimeException("Failed to create enemy from class: " + annotatedClass.getName(), e);
            }
        }
    }

    public Enemy createEnemy() {
        final ThreadLocalRandom random = ThreadLocalRandom.current();
        final int index = random.nextInt(commonEnemiesSuppliers.size());
        return commonEnemiesSuppliers.get(index).get();
    }

    public Enemy createBoss() {
        final ThreadLocalRandom random = ThreadLocalRandom.current();
        final int index = random.nextInt(bossEnemiesSuppliers.size());
        return bossEnemiesSuppliers.get(index).get();
    }
}
