package cz.empatix.assets.creators;

import java.util.Map;

public interface AssetCreator<PRE, POST> {
    POST get(final String name);
    void create(Map<String,PRE> data);

    void destroy();
}
