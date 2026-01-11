package cz.empatix.map.service;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.hud.service.image.AnimatedImage;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.utils.RomanNumber;
import org.joml.Vector2f;
import org.joml.Vector3i;

import java.time.Duration;
import java.time.Instant;
import java.util.List;

public class MapTransitionService {
    private final static Vector3i TEXT_COLOR = new Vector3i(200, 92, 18);
    private final static int DISPLAY_TIME_SECONDS = 3;
    private final static int DISPLAY_TIME_MILLIS = DISPLAY_TIME_SECONDS * 1000;
    private final AnimatedImage title;
    private final CommonText titleText;
    private final CommonText infoText;
    private Instant lastTransition = Instant.now().minus(Duration.ofSeconds(DISPLAY_TIME_SECONDS));
    private int floor = 1;
    public MapTransitionService(){
        this.title = new AnimatedImage("ingame/title.tga", new Vector2f(), 5f, 4, RenderScene.HUD, 6);
        this.title.setPosition(480 - title.getWidth()/2f, 50);

        this.titleText = new CommonText("Floor "+ RomanNumber.toRoman(floor), "PixelFJVerdana12pt.ttf",new Vector2f(), 4, FontSize.FONTSIZE_32);
        this.titleText.setPosition(480 - titleText.getWidth()/2f, 50 + 10);
        this.titleText.setColorRGB(TEXT_COLOR);

        this.infoText = new CommonText("Enemies have increased health by "+(floor-1)*12+"%", "PixelFJVerdana12pt.ttf",new Vector2f(), 4, FontSize.FONTSIZE_22);
        this.infoText.setPosition(480 - infoText.getWidth()/2f, 500);
        this.infoText.setColorRGB(TEXT_COLOR);

    }

    public void nextFloor(){
        floor++;
        titleText.setText("Floor "+ RomanNumber.toRoman(floor));
        infoText.setText("Enemies have increased health by "+(floor-1)*12+"%");
        lastTransition = Instant.now();
    }
    public void update(final float dt){
        title.update(dt);
    }

    public List<Renderable> toRender(){
        if (Duration.between(lastTransition, Instant.now()).toMillis() > DISPLAY_TIME_MILLIS) {
            return List.of();
        }
        return List.of(title, titleText, infoText);
    }
}
