package cz.empatix.assets.creators;



public interface AssetCreator {
    void create();
    void load();
    void preload();
    void destroy();
}
