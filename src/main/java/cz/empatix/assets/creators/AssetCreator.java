package cz.empatix.assets.creators;



public interface AssetCreator {
    void create();
    Runnable load();
    void preload();
    void destroy();
}
