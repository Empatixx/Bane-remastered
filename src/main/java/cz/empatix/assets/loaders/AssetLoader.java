package cz.empatix.assets.loaders;


import cz.empatix.assets.creators.AssetCreator;

import java.util.concurrent.Callable;

public interface AssetLoader<DATA> extends Callable<AssetLoader<?>> {
    int load();
    int preload();
    void submit();

    AssetCreator<DATA,?> getCreator();

}
