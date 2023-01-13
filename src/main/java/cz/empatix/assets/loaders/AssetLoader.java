package cz.empatix.assets.loaders;


import cz.empatix.assets.creators.AssetCreator;
import cz.empatix.render.Shader;

import java.util.Map;
import java.util.concurrent.Callable;

public interface AssetLoader<DATA> extends Callable<AssetLoader<?>> {
    int load();
    AssetCreator<DATA,?> getCreator();

}
