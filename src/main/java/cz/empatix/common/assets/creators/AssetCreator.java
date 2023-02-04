package cz.empatix.common.assets.creators;


public interface AssetCreator {
    boolean isReady();
    void ready();
    void destroy();
}
