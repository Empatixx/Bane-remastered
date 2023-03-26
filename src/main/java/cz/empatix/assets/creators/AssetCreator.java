package cz.empatix.assets.creators;



public interface AssetCreator {
    void create();
    void load();
    default Runnable loadAsync() {
        return this::load;
    }
    void preload();
    void destroy();
}
