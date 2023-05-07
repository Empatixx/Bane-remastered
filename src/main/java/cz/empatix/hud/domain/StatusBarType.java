package cz.empatix.hud.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.textureatlas.domain.TextureAtlas;

public enum StatusBarType {
    HEALTH(0, 0, 20, 20, "textures/ingame/healthBar.tga"),
    ARMOR(0, 0, 20, 20, "textures/ingame/armorBar.tga");
    private final TextureAtlas.Entry tex;
    private final int xOffset;
    private final int yOffset;
    private final int width;
    private final int height;

    StatusBarType(final int xOffset,
                  final int yOffset,
                  final int width,
                  final int height,
                  final String texturePath) {
        this.xOffset = xOffset;
        this.yOffset = yOffset;
        this.width = width;
        this.height = height;
        this.tex = AssetsService.getTexture(texturePath);
    }

    public float getNormalizedXOffset() {
        return (float) xOffset / tex.atlasWidth();
    }
    public float getNormalizedYOffset() {
        return (float) yOffset / tex.atlasHeight();
    }
    public float getNormalizedWidth() {
        return (float) getWidth() / tex.atlasWidth();
    }
    public float getNormalizedHeight() {
        return (float) getHeight() / tex.atlasHeight();
    }
    public int getWidth(){
        return width;
    }
    public int getHeight(){
        return height;
    }

    public TextureAtlas.Entry getTexture() {
        return tex;
    }
}
