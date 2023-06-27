package cz.empatix.minimap.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.MiniMapBinding;
import cz.empatix.rooms.service.*;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;

import java.util.concurrent.ThreadLocalRandom;

public class MinimapRoom implements Drawable {
    private final static float ALPHA_DECREASE = 0.4f;
    private final static int MINIMAP_X_START = 800;
    private final static int MINIMAP_Y_START = 100;
    private final static int DIMENSIONS = 16;
    private final static int SCALE = 2;
    private final int x;
    private final int y;
    private boolean visited;
    private final Animation animation;
    private final Vector2f normSpriteSize;
    private final Vector2f normSubImagePos;
    private final float randomOffset = ThreadLocalRandom.current().nextFloat(10);

    // these are used for centering by current center room
    private int offsetX = 0;
    private int offsetY = 0;

    public MinimapRoom(final int x, final int y, final Room room) {
        this.x = x;
        this.y = y;
        this.visited = false;
        final TextureAtlas.Entry entry = AssetsService.getTexture("test/minimap_icons.tga");
        this.animation = Animation.builder()
                .addState(AnimationState.IDLE, 0, 5, DIMENSIONS, DIMENSIONS, -1)
                .setTotal(1, 5)
                .texture(entry)
                .scale(SCALE)
                .build();
        this.animation.setCurrentFrame(getTextureFrame(room));
        this.normSpriteSize = new Vector2f(16f / entry.atlasWidth(), 16f / entry.atlasHeight());
        this.normSubImagePos = entry.normalizedStartPositions();
    }

    private int getTextureFrame(final Room r) {
        if (r instanceof StartRoom) {
            return 0;
        } else if (r instanceof CommonRoom) {
            return 1;
        } else if (r instanceof TreasureRoom) {
            return 2;
        } else if (r instanceof ShopRoom) {
            return 3;
        } else if (r instanceof BossRoom) {
            return 4;
        }
        throw new IllegalStateException("Unknown room type: " + r.getClass().getSimpleName());
    }

    public void visit() {
        visited = true;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(animation.getVertices(new Vector2f(MINIMAP_X_START + (x - offsetX) * DIMENSIONS * SCALE, MINIMAP_Y_START + (y - offsetY) * DIMENSIONS * SCALE)),
                animation.getTextureId(),
                new MiniMapBinding(Camera.getOriginWithoutZoomProjection(), normSpriteSize, normSubImagePos,
                        System.currentTimeMillis() % 1000000 / 1000f + randomOffset, visited, calculateAlpha(), animation.getCurrentTextureIndex()));
    }

    private float calculateAlpha() {
        return (1 - ALPHA_DECREASE * (Math.abs(x - offsetX) + Math.abs(y - offsetY)));
    }

    public void setOffset(final int x, final int y) {
        this.offsetX = x;
        this.offsetY = y;
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.HUD;
    }

    @Override
    public int getZIndex() {
        return 5;
    }

    @Override
    public boolean canRender() {
        return true;
    }
}
