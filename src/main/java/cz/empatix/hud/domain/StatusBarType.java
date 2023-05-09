package cz.empatix.hud.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector3f;

public enum StatusBarType {
    HEALTH(0, 0, 20, 20, 3,
            new Vector3f(1, 0, 0), "ingame/healthBar.tga"),
    ARMOR(0, 0, 20, 20, 2,
            new Vector3f(0, 0, 1), "ingame/armorBar.tga");
    private final TextureAtlas.Entry tex;
    private final Vector3f color;
    private final int xOffset;
    private final int yOffset;
    private final int width;
    private final int height;
    private final int scale;

    StatusBarType(final int xOffset,
                  final int yOffset,
                  final int width,
                  final int height,
                  final int scale,
                  final Vector3f color,
                  final String texturePath) {
        this.xOffset = xOffset;
        this.yOffset = yOffset;
        this.width = width;
        this.height = height;
        this.color = color;
        this.scale = scale;
        this.tex = AssetsService.getTexture(texturePath);
    }

    public float getNormalizedXOffset() {
        return (float) xOffset / tex.atlasWidth();
    }

    public float getNormalizedYOffset() {
        return (float) yOffset / tex.atlasHeight();
    }

    public float getNormalizedWidth() {
        return (float) getWidth() / tex.width();
    }

    public float getNormalizedHeight() {
        return (float) getHeight() / tex.height();
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    public TextureAtlas.Entry getTexture() {
        return tex;
    }

    public int getScale() {
        return scale;
    }
}
