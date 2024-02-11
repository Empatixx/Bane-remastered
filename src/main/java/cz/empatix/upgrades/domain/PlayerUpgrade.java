package cz.empatix.upgrades.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.ui.TransparentButton;
import org.joml.Vector2f;

import java.util.ArrayList;
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
    private final CommonText title;
    private final List<CommonText> description;
    private final TransparentButton buy;
    private final CommonText price;

    public PlayerUpgrade(final String upgradeImg, final QualityLevel level, final boolean isLocked) {
        position = new Vector2f();
        locked = isLocked;
        upgradeImage = new CommonImage(upgradeImg, new Vector2f(), 2, 3, RenderScene.OVERLAYS);
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
        this.title = new CommonText("", "PixelFJVerdana12pt.ttf", new Vector2f(675, 200), 4,
                RenderScene.OVERLAYS, FontSize.FONTSIZE_22);
        this.title.setColorRGB(222, 112, 38);
        this.description = new ArrayList<>();

        this.buy = new TransparentButton("Buy", new Vector2f(735, 410), 3);
        this.price = new CommonText("100", "PixelFJVerdana12pt.ttf", new Vector2f(840, 410), 4,
                RenderScene.OVERLAYS, FontSize.FONTSIZE_22);
        this.price.setColorRGB(222, 112, 38);
    }

    public void setDescription(final String description) {
        final List<String> lines = description.lines().toList();
        for (int i = 0; i < lines.size(); i++) {
            final CommonText text = new CommonText(lines.get(i), "PixelFJVerdana12pt.ttf", new Vector2f(675, 250 + 16 * (i + 1)), 4,
                    RenderScene.OVERLAYS, FontSize.FONTSIZE_16);
            text.setColorRGB(222, 112, 38);
            text.setCenterX(675, 915);
            this.description.add(text);
        }
    }

    public String getDescription() {
        final StringBuilder sb = new StringBuilder();
        for (final CommonText text : description) {
            sb.append(text.getText()).append("\n");
        }
        return sb.toString();
    }

    public void setTitle(final String title) {
        this.title.setText(title);
        this.title.setCenterX(675, 915);
    }

    public String getTitle() {
        return title.getText();
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
        final List<Renderable> renderables = new ArrayList<>();
        renderables.add(upgradeImage);
        return renderables;
    }

    public List<Renderable> getCurrentRenderables() {
        final List<Renderable> renderables = new ArrayList<>(title.getSubRenderables());
        renderables.add(buy);
        for (final CommonText text : description) {
            renderables.addAll(text.getSubRenderables());
        }
        renderables.addAll(price.getSubRenderables());
        return renderables;
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        buy.update(dt, mouseX, mouseY);
    }
}
