package cz.empatix.textureatlas.domain;

import org.joml.Vector2i;

import java.util.HashMap;
import java.util.Map;

public record TextureAtlasBlueprint (
        int targetSize,
        Map<String, Vector2i> startPositions
){
    public TextureAtlasBlueprint(final int targetSize) {
        this(targetSize, new HashMap<>());
    }
    public int area(){
        return targetSize * targetSize;
    }
    public void addTexture(final String name, final int x, final int y){
        startPositions.put(name, new Vector2i(x, y));
    }
    public int getX(final String name){
        return startPositions.get(name).x;
    }
    public int getY(final String name){
        return startPositions.get(name).y;
    }
}
