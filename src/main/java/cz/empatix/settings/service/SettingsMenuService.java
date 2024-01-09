package cz.empatix.settings.service;

import cz.empatix.hud.service.CommonBackground;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.ui.CommonButton;
import cz.empatix.ui.StickyHoverButton;
import cz.empatix.ui.TransparentButton;
import cz.empatix.ui.UISelector;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

public class SettingsMenuService {
    private final SettingsService settingsService = SettingsService.getInstance();
    private final CommonBackground background;
    private final List<CommonButton> buttons;
    private final UISelector selector;
    private boolean active;

    public SettingsMenuService() {
        background = new CommonBackground("menu/settings_background.tga", 480, 320);

        final CommonButton applyButton = new CommonButton("Apply", new Vector2f(800, 160), 1.5f);
        //applyButton.onClick(() -> settingsService.apply());

        final CommonButton revertButton = new CommonButton("Revert", new Vector2f(800, 240), 1.5f);
        //revertButton.onClick(() -> settingsService.revert());

        final CommonButton exitButton = new CommonButton("Exit", new Vector2f(800, 320), 1.5f);
        exitButton.onClick(() -> activate(false));

        buttons = List.of(exitButton, revertButton, applyButton);

        selector = new UISelector();
        final StickyHoverButton video = new StickyHoverButton("Video", "menu/settings_menubar.tga", 30, 16, new Vector2f(325, 160), 5f);
        final StickyHoverButton audio = new StickyHoverButton("Audio", "menu/settings_menubar.tga", 30, 16, new Vector2f(480, 160), 5);
        final StickyHoverButton controls = new StickyHoverButton("Controls", "menu/settings_menubar.tga", 30, 16, new Vector2f(635, 160), 5);
        selector.addElements(List.of(video, audio, controls));
    }

    public List<Renderable> toRender(final double dt) {
        if (!active) {
            return List.of();
        }
        final List<Renderable> renderables = new ArrayList<>();
        renderables.add(background);
        renderables.addAll(buttons);
        renderables.addAll(selector.toRender(dt));
        return renderables;
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        background.update(dt);
        buttons.forEach(button -> button.update(dt, mouseX, mouseY));
        selector.update(dt, mouseX, mouseY);
    }

    public void activate(final boolean active) {
        this.active = active;
    }

    public boolean isActive() {
        return active;
    }

    public void mousePressed(int button, float mouseX, float mouseY) {
        buttons.forEach(button1 -> button1.mousePressed(button, mouseX, mouseY));
        selector.mousePressed(button, mouseX, mouseY);
    }
}
