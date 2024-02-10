package cz.empatix.upgrades.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

import java.util.List;

public class PlayerUpgrade implements Drawable {
    private final static int SCALE = 2;
    private final static int DIMENSION_WIDTH = 32 * SCALE;
    private final static int DIMENSION_HEIGHT = 32 * SCALE;
    private final Animation animation;
    private final Vector2f position;
    private boolean locked;
    private final CommonImage upgradeImage;
    private final String upgradeImg;

    public PlayerUpgrade(final String upgradeImg, final QualityLevel level, final boolean isLocked) {
        position = new Vector2f();
        locked = isLocked;
        upgradeImage = new CommonImage(upgradeImg, new Vector2f(),2, 3, RenderScene.OVERLAYS);
        this.upgradeImg = upgradeImg;
        animation = Animation.builder()
                .texture(AssetsService.getTexture("ingame/talent_frame.tga"))
                .setTotal(6, 1)
                .addState(AnimationState.TYPE0, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.TYPE1, 1, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.TYPE2, 2, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.TYPE3, 3, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.TYPE4, 4, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.TYPE5, 5, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .build();
        if (isLocked) {
            switch (level) {
                case COMMON -> animation.setState(AnimationState.TYPE1);
                case RARE -> animation.setState(AnimationState.TYPE3);
                case EPIC -> animation.setState(AnimationState.TYPE5);
            }
        } else {
            switch (level) {
                case COMMON -> animation.setState(AnimationState.TYPE0);
                case RARE -> animation.setState(AnimationState.TYPE2);
                case EPIC -> animation.setState(AnimationState.TYPE4);
            }
        }
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonBinding(Camera.getOriginWithoutZoomProjection())
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.OVERLAYS;
    }

    @Override
    public int getZIndex() {
        return 3;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
        upgradeImage.setPosition(x + 16, y + 16);
    }

    public boolean isLocked() {
        return locked;
    }

    public boolean isPressed(final float mouseX, final float mouseY) {
        return mouseX >= position.x() && mouseX <= position.x() + DIMENSION_WIDTH
                && mouseY >= position.y() && mouseY <= position.y() + DIMENSION_HEIGHT;
    }

    public PlayerUpgrade deepCopy() {
        final PlayerUpgrade copy = new PlayerUpgrade(upgradeImg, QualityLevel.COMMON, locked);
        copy.setPosition(position.x(), position.y());
        return copy;
    }

    public int getWidth() {
        return DIMENSION_WIDTH;
    }

    public int getHeight() {
        return DIMENSION_HEIGHT;
    }
    @Override
    public List<Renderable> getSubRenderables() {
        return List.of(upgradeImage);
    }
}
