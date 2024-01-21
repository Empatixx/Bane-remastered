package cz.empatix.tiles.service;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Random;

public class PerlinNoiseGenerator {
    private static final int SIZE = 256;
    private static final double[][] GRAD2 = {
            {1, 0}, {-1, 0}, {0, 1}, {0, -1},
            {1, 1}, {-1, 1}, {1, -1}, {-1, -1}
    };
    private final ArrayList<Integer> p = new ArrayList<>();

    public PerlinNoiseGenerator(final long seed) {
        // Initialize a Random object with the provided seed
        final Random random = new Random(seed);

        // Fill permutation list
        for (int i = 0; i < SIZE; i++) {
            p.add(i);
        }

        // Shuffle using Collections and the Random instance with a seed
        Collections.shuffle(p, random);
        // Duplicate the list to avoid using modulo operation for wrapping
        p.addAll(new ArrayList<>(p));
    }

    private double dot(final double[] g, final double x, final double y) {
        return g[0] * x + g[1] * y;
    }

    private double lerp(final double a, final double b, final double t) {
        return a + t * (b - a);
    }

    private double fade(final double t) {
        return t * t * t * (t * (t * 6 - 15) + 10);
    }

    public double noise(final double x, final double y) {
        int X = (int) Math.floor(x) & 255;
        int Y = (int) Math.floor(y) & 255;

        double xf = x - Math.floor(x);
        double yf = y - Math.floor(y);

        double u = fade(xf);
        double v = fade(yf);

        int aa = p.get(p.get(X) + Y);
        int ab = p.get(p.get(X) + Y + 1);
        int ba = p.get(p.get(X + 1) + Y);
        int bb = p.get(p.get(X + 1) + Y + 1);

        double x1 = lerp(dot(GRAD2[aa & 7], xf, yf),
                dot(GRAD2[ba & 7], xf - 1, yf), u);
        double x2 = lerp(dot(GRAD2[ab & 7], xf, yf - 1),
                dot(GRAD2[bb & 7], xf - 1, yf - 1), u);

        return lerp(x1, x2, v);
    }
}
