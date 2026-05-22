package cz.empatix.tiles.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class PerlinNoiseGeneratorTest {

    @Test
    void sameSeedProducesIdenticalNoise() {
        final PerlinNoiseGenerator a = new PerlinNoiseGenerator(42L);
        final PerlinNoiseGenerator b = new PerlinNoiseGenerator(42L);

        for (double x = 0; x < 10; x += 0.37) {
            for (double y = 0; y < 10; y += 0.53) {
                assertEquals(a.noise(x, y), b.noise(x, y),
                        "Same seed must give identical noise at (" + x + "," + y + ")");
            }
        }
    }

    @Test
    void isDeterministicAcrossRepeatedCalls() {
        final PerlinNoiseGenerator gen = new PerlinNoiseGenerator(7L);
        final double first = gen.noise(3.14, 2.71);
        final double second = gen.noise(3.14, 2.71);
        assertEquals(first, second);
    }

    @Test
    void differentSeedsGenerallyDiffer() {
        final PerlinNoiseGenerator a = new PerlinNoiseGenerator(1L);
        final PerlinNoiseGenerator b = new PerlinNoiseGenerator(2L);

        boolean foundDifference = false;
        for (double x = 0; x < 8 && !foundDifference; x += 0.5) {
            for (double y = 0; y < 8 && !foundDifference; y += 0.5) {
                if (a.noise(x, y) != b.noise(x, y)) {
                    foundDifference = true;
                }
            }
        }
        assertTrue(foundDifference, "Different seeds should produce at least one differing sample");
    }

    @Test
    void noiseStaysWithinPerlinBounds() {
        final PerlinNoiseGenerator gen = new PerlinNoiseGenerator(123L);
        // Classic Perlin noise output is bounded roughly within [-1, 1].
        for (double x = -5; x < 5; x += 0.31) {
            for (double y = -5; y < 5; y += 0.29) {
                final double v = gen.noise(x, y);
                assertTrue(v >= -1.5 && v <= 1.5,
                        "noise out of expected range at (" + x + "," + y + "): " + v);
            }
        }
    }

    @Test
    void noiseAtIntegerLatticePointsIsZero() {
        // Perlin gradient noise evaluates to 0 at integer lattice coordinates.
        final PerlinNoiseGenerator gen = new PerlinNoiseGenerator(999L);
        assertEquals(0.0, gen.noise(0, 0), 1e-9);
        assertEquals(0.0, gen.noise(3, 4), 1e-9);
        assertEquals(0.0, gen.noise(10, 7), 1e-9);
    }
}
