package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.*;
import org.reflections.Reflections;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.time.Duration;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.concurrent.BlockingDeque;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.atomic.AtomicReference;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;
import java.util.function.Supplier;

public class EnemyFactory {
    private static final Logger LOGGER = LoggerFactory.getLogger(EnemyFactory.class);
    private final static AtomicReference<List<EnemySupplier>> commonEnemiesSuppliers = new AtomicReference<>();
    private final static AtomicReference<List<EnemySupplier>> bossEnemiesSuppliers = new AtomicReference<>();

    public static void init() {
        CompletableFuture.runAsync(() -> {
            final Reflections reflections = new Reflections("cz.empatix.enemies.domain");
            final Set<Class<?>> annotatedClasses = reflections.getTypesAnnotatedWith(SpawnableEnemy.class);
            final List<EnemySupplier> commonEnemiesSuppliers = new ArrayList<>();
            final List<EnemySupplier> bossEnemiesSuppliers = new ArrayList<>();

            final Instant start = Instant.now();

            LOGGER.info("Found {} enemies", annotatedClasses.size());
            for (Class<?> annotatedClass : annotatedClasses) {
                final SpawnableEnemy annotation = annotatedClass.getAnnotation(SpawnableEnemy.class);
                try {
                    final Supplier<Enemy> supplier = () -> {
                        try {
                            return (Enemy) annotatedClass.getConstructor().newInstance();
                        } catch (Exception e) {
                            LOGGER.error("Failed to create enemy from class: " + annotatedClass.getName(), e);
                            throw new RuntimeException(e);
                        }
                    };
                    final int minFloor = annotation.minFloor();
                    final int maxFloor = annotation.maxFloor();
                    final EnemySupplier enemySupplier = new EnemySupplier(supplier, minFloor, maxFloor);
                    switch (annotation.type()) {
                        case COMMON -> commonEnemiesSuppliers.add(enemySupplier);
                        case BOSS -> bossEnemiesSuppliers.add(enemySupplier);
                    }
                } catch (Exception e) {
                    throw new RuntimeException("Failed to create enemy from class: " + annotatedClass.getName(), e);
                }
            }
            LOGGER.info("Loaded total common enemies: {}", commonEnemiesSuppliers.size());
            LOGGER.info("Loaded total boss enemies: {}", bossEnemiesSuppliers.size());
            EnemyFactory.bossEnemiesSuppliers.set(bossEnemiesSuppliers);
            EnemyFactory.commonEnemiesSuppliers.set(commonEnemiesSuppliers);

            final Instant end = Instant.now();
            LOGGER.info("EnemyFactory initialized in: {} ms", Duration.between(start, end).toMillis());
        });
    }

    public static Enemy createEnemy(final int currentFloor) {
        final ThreadLocalRandom random = ThreadLocalRandom.current();
        final List<EnemySupplier> possibleSuppliers = commonEnemiesSuppliers.get().stream()
                .filter(supplier -> supplier.isSpawnable(currentFloor))
                .toList();
        if (possibleSuppliers.isEmpty()) {
            throw new RuntimeException("No enemies available for floor: " + currentFloor);
        }
        final int index = random.nextInt(possibleSuppliers.size());
        return possibleSuppliers.get(index).supplier().get();
    }

    public static Enemy createBoss(final int currentFloor) {
        final ThreadLocalRandom random = ThreadLocalRandom.current();
        final List<EnemySupplier> possibleSuppliers = bossEnemiesSuppliers.get().stream()
                .filter(supplier -> supplier.isSpawnable(currentFloor))
                .toList();
        if (possibleSuppliers.isEmpty()) {
            throw new RuntimeException("No enemies available for floor: " + currentFloor);
        }
        final int index = random.nextInt(possibleSuppliers.size());
        return possibleSuppliers.get(index).supplier().get();
    }
}
