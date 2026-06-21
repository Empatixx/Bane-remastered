package cz.empatix.common;

import java.util.List;
import java.util.Random;

/**
 * Seeded source of randomness for gameplay. Wrapping {@link Random} (a specified, reproducible
 * algorithm) means the same seed always replays the same run — the foundation for daily runs,
 * replays and future multiplayer determinism. Created once per run with a recorded seed and injected
 * where randomness is needed, rather than reached through a global singleton.
 *
 * <p>The API mirrors the call shapes already used across the codebase so migrating off
 * {@code ThreadLocalRandom} is a near drop-in.
 */
public class RngService {

    private final long seed;
    private final Random random;

    public RngService(final long seed) {
        this.seed = seed;
        this.random = new Random(seed);
    }

    /** The seed this generator was created with — record/display it to enable replays and dailies. */
    public long seed() {
        return seed;
    }

    /** A value in {@code [0, bound)}. */
    public int nextInt(final int bound) {
        return random.nextInt(bound);
    }

    /** A value in {@code [origin, bound)}. */
    public int nextInt(final int origin, final int bound) {
        return origin + random.nextInt(bound - origin);
    }

    /** A value in {@code [0, 1)}. */
    public float nextFloat() {
        return random.nextFloat();
    }

    /** A value in {@code [0, 1)}. */
    public double nextDouble() {
        return random.nextDouble();
    }

    public boolean nextBoolean() {
        return random.nextBoolean();
    }

    /** A uniformly random element of {@code list}. */
    public <T> T pick(final List<T> list) {
        return list.get(random.nextInt(list.size()));
    }
}
