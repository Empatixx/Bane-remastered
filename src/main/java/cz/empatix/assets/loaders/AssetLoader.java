package cz.empatix.assets.loaders;


import java.util.concurrent.Callable;

public interface AssetLoader extends Callable<Void> {
    int load();
    int preload();
}
