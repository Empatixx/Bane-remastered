package cz.empatix.ui;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlasInfo;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3i;
import org.lwjgl.assimp.AIQuaternion;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

public class Slider implements UIElement {
    private final static Consumer<Float> EMPTY_CONSUMER = e -> {};
    private final static int SCALE = 2;
    private final Vector2f position;
    private final Animation animation;
    private final CommonImage cursor;
    private final CommonText valueText;
    private final Vector2i dimensions;
    private float value;
    private boolean selected;
    private Consumer<Float> onChange = EMPTY_CONSUMER;

    public Slider() {
        this.position = new Vector2f();
        final TextureAtlas.Entry entry = AssetsService.getTexture("menu/slider.tga");
        this.animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.STATIC, 0, 1, 128, 8, -1)
                .texture(entry)
                .scale(SCALE)
                .build();
        this.dimensions = new Vector2i(entry.width(), entry.height()).mul(SCALE);
        animation.setState(AnimationState.STATIC);
        this.cursor = new CommonImage("menu/slider-button.tga", new Vector2f(0, 0), 2.f, 2);
        cursor.setPosition(position.x(), position.y() - cursor.getHeight() / 4f);

        this.valueText = new CommonText("0%", "PixelFJVerdana12pt.ttf", new Vector2f(), 1);
        value = 0;
    }
    public void setTextPosition(final float x, final float y) {
        valueText.setPosition(x, y);
    }
    public void setCenterTextY(final float yMin, final float yMax) {
        valueText.setCenterY(yMin, yMax);
    }
    public void setTextRGB(final Vector3i rgb) {
        valueText.setColorRGB(rgb);
    }


    public void setPosition(final float x, final float y) {
        this.position.set(x, y);
        this.cursor.setPosition(x, y - cursor.getHeight() / 4f);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonBinding(Camera.getOriginProjection())
        );
    }

    @Override
    public List<Renderable> getSubRenderables() {
        final List<Renderable> renderables = new ArrayList<>();
        renderables.add(cursor);
        renderables.addAll(valueText.getSubRenderables());
        return renderables;
    }

    @Override
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public void update(double dt, double mouseX, double mouseY) {
        if (selected) {
            cursor.setPosition((float)mouseX - cursor.getWidth() / 2f, cursor.getPosition().y());
            if (cursor.getPosition().x() < position.x()) {
                cursor.setPosition(position.x(), cursor.getPosition().y());
            }
            if (cursor.getPosition().x() > position.x() + dimensions.x() - cursor.getWidth()) {
                cursor.setPosition(position.x() + dimensions.x() - cursor.getWidth(), cursor.getPosition().y());
            }
            value = (cursor.getPosition().x() - position.x()) / (dimensions.x() - cursor.getWidth());
            valueText.setText((int)(value * 100) + "%");
            onChange.accept(value);
        }
    }

    @Override
    public void mousePressed(int button, double mouseX, double mouseY) {
        if (intersectsCursor(mouseX, mouseY)) {
            selected = true;
        }
    }
    public void onChange(final Consumer<Float> onChange) {
        this.onChange = onChange;
    }

    public boolean intersectsCursor(final double mouseX, final double mouseY){
        return cursor.getPosition().x() < mouseX && cursor.getPosition().x() + cursor.getWidth() > mouseX &&
                cursor.getPosition().y() < mouseY && cursor.getPosition().y() + cursor.getHeight() > mouseY;
    }

    @Override
    public void mouseReleased(int button, double mouseX, double mouseY) {
        selected = false;
    }

    @Override
    public void keyPressed(int key) {

    }

    @Override
    public void keyReleased(int key) {

    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }

    @Override
    public boolean isHovered() {
        return false;
    }

    public Vector2f getPosition() {
        return position;
    }

    public int getHeight() {
        return dimensions.y();
    }

    public void setValue(final float value) {
        this.value = value;
        cursor.setPosition(position.x() + value * (dimensions.x() - cursor.getWidth()), cursor.getPosition().y());
        valueText.setText((int)(value * 100) + "%");
    }
}
