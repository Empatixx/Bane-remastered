package cz.empatix.core;


import java.util.List;
import java.util.Queue;

public interface AssetLoader<A> {
    int load();
    int preload();
    Queue<A> getAssets();
}
