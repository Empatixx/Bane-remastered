package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.*;
import cz.empatix.enemies.domain.bosses.Golem;
import cz.empatix.enemies.domain.bosses.KingSlime;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Supplier;

public class EnemyFactory {
    private final List<Supplier<Enemy>> commonEnemiesSuppliers = List.of(Bat::new, Slime::new, DemonEye::new, Ghost::new, RedSlime::new, Snake::new, Rat::new,
            EyeWatcher::new, Spider::new);
    private final List<Supplier<Enemy>> bossEnemiesSuppliers = List.of(Golem::new);//KingSlime::new

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
