package cz.empatix.assets.creators;

import java.util.Map;

public interface AssetCreator<PRE, POST> {
    POST get(final String name);
    void add(Map<String,PRE> data);
    boolean isReady();
    void ready();
    void destroy();
}
