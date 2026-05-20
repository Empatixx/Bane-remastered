package cz.empatix.keyboard.domain;

public enum KeyboardType {
    Q("q"),
    W("w"),
    E("e"),
    R("r"),
    A("a"),
    S("s"),
    D("d"),
    F("f"),
    TAB("tab"),
    CAPS_LOCK("capslock"),
    NUMBER_ONE("1"),
    NUMBER_TWO("2"),
    NUMBER_THREE("3"),
    NUMBER_FOUR("4"),
    NUMBER_FIVE("5");
    private final String texturePath;
    KeyboardType(final String texturePath){
        this.texturePath = texturePath;
    }
    public String getTexturePath(){
        return "ingame/keyboard/" + texturePath + ".tga";
    }
}
