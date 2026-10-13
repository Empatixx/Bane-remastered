package cz.empatix.items.domain;

import cz.empatix.common.RngService;

import java.util.stream.Stream;

public enum ItemRarity {
    COMMON(60),
    UNCOMMON(25),
    RARE(15),
    UNDEFINED(0);
    private final int weight;
    ItemRarity(final int weight){
        this.weight = weight;
    }

    public int getWeight() {
        return weight;
    }

    public static ItemRarity randomRarity(final RngService rng, final ItemRarity... itemRarities){
        final int totalWeight = Stream.of(itemRarities)
                .mapToInt(ItemRarity::getWeight)
                .sum();
        int randomValue = rng.nextInt(totalWeight);
        for (final ItemRarity rarity : itemRarities) {
            randomValue -= rarity.getWeight();
            if (randomValue < 0) {
                return rarity;
            }
        }
        return UNDEFINED;
    }

}
