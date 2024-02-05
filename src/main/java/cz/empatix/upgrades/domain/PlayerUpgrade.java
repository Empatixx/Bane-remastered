package cz.empatix.upgrades.domain;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

public class PlayerUpgrade implements Drawable {
    private CommonImage image;
    public PlayerUpgrade(){
        image = new CommonImage("ingame/talentframe1.tga", new Vector2f(0, 0), 2f);
    }
    public PlayerUpgrade(final String txt){
        image = new CommonImage(txt, new Vector2f(0, 0), 2f);
    }
    @Override
    public RenderData renderData() {
        return image.renderData();
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
    public void setPosition(final float x, final float y){
        image.setPosition(x, y);
    }
}
