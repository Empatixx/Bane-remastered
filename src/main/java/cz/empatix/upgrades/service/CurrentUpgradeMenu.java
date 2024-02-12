package cz.empatix.upgrades.service;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.ui.TransparentButton;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;

public class CurrentUpgradeMenu {
    private static final AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final CommonText title;
    private final List<CommonText> description;
    private final TransparentButton buy;
    private final TransparentButton bought;
    private final TransparentButton cantBuy;
    private final CommonText price;
    private final CommonImage upgradeBackground;
    private PlayerUpgrade selectedUpgrade;
    private PlayerUpgrade currentUpgrade;

    public CurrentUpgradeMenu() {
        upgradeBackground = new CommonImage("ingame/pausebg.tga", new Vector2f(675, 150), 3.0f, 2, RenderScene.OVERLAYS);

        this.title = new CommonText("", "PixelFJVerdana12pt.ttf", new Vector2f(675, 200), 4,
                RenderScene.OVERLAYS, FontSize.FONTSIZE_22);
        this.title.setColorRGB(222, 112, 38);
        this.description = new ArrayList<>();

        this.buy = new TransparentButton("Buy", new Vector2f(735, 410), 3);
        this.bought = new TransparentButton("Bought", new Vector2f(750, 410), 3);
        this.cantBuy = new TransparentButton("Can't buy", new Vector2f(750, 410), 3);
        this.price = new CommonText("100,-", "PixelFJVerdana12pt.ttf", new Vector2f(840, 410), 4,
                RenderScene.OVERLAYS, FontSize.FONTSIZE_22);
        this.price.setColorRGB(222, 112, 38);
    }
    public List<Renderable> toRender() {
        if (currentUpgrade == null) {
            return List.of();
        }
        final List<Renderable> renderables = new ArrayList<>(title.getSubRenderables());
        if (!currentUpgrade.isBought()) {
            if (currentUpgrade.canBuy()) {
                renderables.add(buy);
                renderables.addAll(price.getSubRenderables());
            } else {
                renderables.addAll(cantBuy.getSubRenderables());
            }
        } else {
            renderables.add(bought);
        }
        renderables.add(upgradeBackground);
        renderables.add(selectedUpgrade);
        for (final CommonText text : description) {
            renderables.addAll(text.getSubRenderables());
        }
        return renderables;
    }
    public void setCurrent(final PlayerUpgrade upgrade){
        currentUpgrade = upgrade;
        if (upgrade == null) {
            return;
        }
        selectedUpgrade = upgrade.deepCopy();
        selectedUpgrade.setPosition(675 + upgradeBackground.getWidth() / 2f - selectedUpgrade.getWidth() / 2f, 90);
        this.title.setText(selectedUpgrade.getTitle());
        this.title.setCenterX(675, 915);

        description.clear();

        final String[] split = upgrade.getDescription().split("\n");
        for (int i = 0; i < split.length; i++) {
            final CommonText text = new CommonText(split[i], "PixelFJVerdana12pt.ttf", new Vector2f(675, 250 + i * 22), 4,
                    RenderScene.OVERLAYS, FontSize.FONTSIZE_16);
            text.setColorRGB(222, 112, 38);
            text.setCenterX(675, 915);
            description.add(text);
        }

        price.setText(upgrade.getPrice() + ",-");

        buy.onClick(() -> {
            if (currentUpgrade != null) {
                if (!currentUpgrade.isBought()) {
                    currentUpgrade.setBought(true);
                    selectedUpgrade.setBought(true);
                    AUDIO_SERVICE.play(SoundEffect.BUY);
                }
            }
        });
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        buy.update(dt, mouseX, mouseY);
    }
    public boolean mousePressed(final int button, final float mouseX, final float mouseY) {
        if (currentUpgrade == null) {
            return false;
        }
        if (currentUpgrade.isBought() || !currentUpgrade.canBuy()) {
            return false;
        }
        buy.mousePressed(button, mouseX, mouseY);
        return buy.isHovered();
    }
}
