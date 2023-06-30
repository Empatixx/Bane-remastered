package cz.empatix.death.service;

import cz.empatix.hud.service.Image;
import cz.empatix.hud.service.Text;
import cz.empatix.postprocessing.domain.Fade;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class DeathService {
    private boolean active = false;
    private boolean continuePressed = false;
    private final List<Image> icons;
    private final List<Text> texts;
    private final Text continueText;
    private final Runnable menuStateAction;
    public DeathService(final Runnable menuStateAction) {
        this.menuStateAction = menuStateAction;

        final Image accuracyIcon = new Image("ingame/accuracy_icon.tga", new Vector2f(250, 200), 1f, 2, RenderScene.OVERLAYS);
        final Image killsIcon = new Image("ingame/kills_icon.tga", new Vector2f(250, 250), 1f, 2, RenderScene.OVERLAYS);
        final Image lootIcon = new Image("ingame/loot_icon.tga", new Vector2f(250, 300), 1f, 2, RenderScene.OVERLAYS);
        final Image floorIcon = new Image("ingame/floor_icon.tga", new Vector2f(250, 350), 1f, 2, RenderScene.OVERLAYS);

        final Image skullImage = new Image("ingame/skull.tga", new Vector2f(429, 50), 6f, 2, RenderScene.OVERLAYS);

        icons = List.of(accuracyIcon, killsIcon, lootIcon, floorIcon, skullImage);

        final Text accuracyText = new Text("Accuracy: 100%", "PixelFJVerdana12pt.ttf", new Vector2f(300, 225), 2, RenderScene.OVERLAYS);
        final Text killsText = new Text("Kills: 0", "PixelFJVerdana12pt.ttf", new Vector2f(300, 275), 2, RenderScene.OVERLAYS);
        final Text lootText = new Text("Loot: 0", "PixelFJVerdana12pt.ttf", new Vector2f(300, 325), 2, RenderScene.OVERLAYS);
        final Text floorText = new Text("Floor: 1", "PixelFJVerdana12pt.ttf", new Vector2f(300, 375), 2, RenderScene.OVERLAYS);

        final Text accuracyCoins = new Text("+0", "PixelFJVerdana12pt.ttf", new Vector2f(650, 225), 2, RenderScene.OVERLAYS);
        final Text killsCoins = new Text("+0", "PixelFJVerdana12pt.ttf", new Vector2f(650, 275), 2, RenderScene.OVERLAYS);
        final Text lootCoins = new Text("+0", "PixelFJVerdana12pt.ttf", new Vector2f(650, 325), 2, RenderScene.OVERLAYS);
        final Text floorCoins = new Text("+0", "PixelFJVerdana12pt.ttf", new Vector2f(650, 375), 2, RenderScene.OVERLAYS);

        continueText = new Text("Press any key to continue...", "PixelFJVerdana12pt.ttf", new Vector2f(700, 500), 2, RenderScene.OVERLAYS);

        accuracyCoins.setColorRGB(212,168,8);
        killsCoins.setColorRGB(212,168,8);
        lootCoins.setColorRGB(212,168,8);
        floorCoins.setColorRGB(212,168,8);

        texts = List.of(accuracyText, killsText, lootText, floorText, accuracyCoins, killsCoins, lootCoins, floorCoins);

    }
    public void update(final float dt, final boolean isPlayerDead){
        if(isPlayerDead && !active){
            Fade.setTween(0.2f);
            active = true;
            return;
        }
        if(!continuePressed && Fade.getValue() > 1){
            Fade.setTween(0);
        } else if (continuePressed && Fade.getValue() > 2){
            menuStateAction.run();
            Fade.setTween(-1);
        }
    }

    public List<Renderable> toRender() {
        if(!active || continuePressed){
            return List.of();
        }
        final List<Renderable> renderables = Stream.of(icons, texts
                        .stream()
                        .map(Text::getSubRenderables)
                        .flatMap(List::stream)
                        .toList())
                .flatMap(List::stream)
                .collect(Collectors.toList());
        if(System.currentTimeMillis() / 200 % 2 == 0){
            renderables.addAll(continueText.getSubRenderables());
        }
        return List.copyOf(renderables);
    }

    public void keyPressed(final int key) {
        if(active && !continuePressed){
            continuePressed = true;
            Fade.setTween(1);
        }
    }
    public boolean isActive(){
        return active;
    }
}
