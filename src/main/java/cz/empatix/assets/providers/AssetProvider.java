package cz.empatix.assets.providers;



public interface AssetProvider {
    void create();
    void load();
    void preload();
    void destroy();
}
