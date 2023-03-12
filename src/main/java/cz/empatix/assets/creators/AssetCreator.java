package cz.empatix.assets.creators;


import java.util.concurrent.Callable;

public interface AssetCreator {
    void create();
    Callable<Void> load();
    void destroy();
}
