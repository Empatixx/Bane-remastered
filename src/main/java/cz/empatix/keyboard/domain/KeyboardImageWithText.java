package cz.empatix.keyboard.domain;

import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector3i;

import java.util.List;

public record KeyboardImageWithText(
        KeyboardImage image,
        AlternativeViewText text
) implements Drawable {

    public void update(double dt) {
        image.update(dt);
        text.update(dt);
    }

    @Override
    public RenderData renderData() {
        return image.renderData();
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return image.getZIndex();
    }

    @Override
    public boolean canRender() {
        return image.canRender();
    }
    @Override
    public List<Renderable> getSubRenderables(){
        return text.getSubRenderables();
    }
    public static KeyboardImageWithText of(final KeyboardImage image, final AlternativeViewText text){
        return new KeyboardImageWithText(image, text);
    }

    public void setPosition(final float x, final float y) {
        this.text.setPosition(x + 50,y + 30);
        this.image.setPosition(x,y);
    }

    public void setColor(Vector3i textColor) {
        text.setColorRGB(textColor);
    }
}
