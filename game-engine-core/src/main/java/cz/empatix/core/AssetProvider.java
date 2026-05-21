package cz.empatix.core;



public interface AssetProvider {
    void create();
    void load();
    void preload();
    void destroy();
}
