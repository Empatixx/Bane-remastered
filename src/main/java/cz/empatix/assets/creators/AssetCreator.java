package cz.empatix.assets.creators;


import java.util.concurrent.Callable;

public interface AssetCreator {
    void create();
    Callable<Integer> load();
    void preload();
    void destroy();
}
