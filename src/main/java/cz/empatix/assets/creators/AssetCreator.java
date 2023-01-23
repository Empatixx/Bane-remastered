package cz.empatix.assets.creators;


public interface AssetCreator {
    boolean isReady();
    void ready();
    void destroy();
}
