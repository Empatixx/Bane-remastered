package cz.empatix.items.domain;

public enum ShopType {
    OFFENSIVE,
    AMMUNITION,
    DEFENSIVE,
    ;

    public float getMultiplier() {
        return switch(this){
            case OFFENSIVE -> 2;
            case AMMUNITION -> 1;
            case DEFENSIVE -> 1.5f;
        };
    }
}
