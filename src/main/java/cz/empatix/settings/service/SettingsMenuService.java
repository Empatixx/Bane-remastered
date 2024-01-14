package cz.empatix.settings.service;

import cz.empatix.hud.service.CommonBackground;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.ui.*;
import org.joml.Vector2f;
import org.joml.Vector3i;

import java.util.ArrayList;
import java.util.List;

public class SettingsMenuService {
    private final static Vector3i TEXT_COLOR = new Vector3i(200, 92, 18);

    private final SettingsService settingsService = SettingsService.getInstance();
    private final CommonBackground background;
    private final List<UIElement> elements;
    private final List<HUDElement> hudElements;
    private final UISelector uiSelector;
    private boolean active;

    public SettingsMenuService() {
        background = new CommonBackground("menu/settings_background.tga", 480, 320);

        final CommonButton applyButton = new CommonButton("Apply", new Vector2f(800, 160), 1.5f);
        //applyButton.onClick(() -> settingsService.apply());

        final CommonButton revertButton = new CommonButton("Revert", new Vector2f(800, 240), 1.5f);
        //revertButton.onClick(() -> settingsService.revert());

        final CommonButton exitButton = new CommonButton("Exit", new Vector2f(800, 320), 1.5f);
        exitButton.onClick(() -> activate(false));

        final Checkbox vsyncCheckbox = new Checkbox();
        vsyncCheckbox.setPosition(400, 225);

        final CommonText vsyncLabel = new CommonText("V-sync:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        vsyncLabel.setPosition(275, 225);
        vsyncLabel.setCenterY(vsyncCheckbox.getPosition().y(), vsyncCheckbox.getPosition().y() + vsyncCheckbox.getHeight());
        vsyncLabel.setColorRGB(TEXT_COLOR);

        final Checkbox fullscreenCheckbox = new Checkbox();
        fullscreenCheckbox.setPosition(400, 275);

        final CommonText fullscreenLabel = new CommonText("Fullscreen:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        fullscreenLabel.setPosition(275, 275);
        fullscreenLabel.setCenterY(fullscreenCheckbox.getPosition().y(), fullscreenCheckbox.getPosition().y() + fullscreenCheckbox.getHeight());
        fullscreenLabel.setColorRGB(TEXT_COLOR);

        final CommonText resolutionLabel = new CommonText("Resolution:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        resolutionLabel.setPosition(275, 325);
        resolutionLabel.setColorRGB(TEXT_COLOR);

        final UISelector selector = new UISelector();
        final StickyHoverButton video = new StickyHoverButton("Video", "menu/settings_menubar.tga", 30, 16, new Vector2f(325, 160), 5f);
        final StickyHoverButton audio = new StickyHoverButton("Audio", "menu/settings_menubar.tga", 30, 16, new Vector2f(480, 160), 5);
        final StickyHoverButton controls = new StickyHoverButton("Controls", "menu/settings_menubar.tga", 30, 16, new Vector2f(635, 160), 5);
        selector.addElements(List.of(video, audio, controls));

        uiSelector = selector;
        elements = List.of(exitButton, revertButton, applyButton, vsyncCheckbox, fullscreenCheckbox);
        hudElements = List.of(vsyncLabel, fullscreenLabel, resolutionLabel);
    }

    public List<Renderable> toRender(final double dt) {
        if (!active) {
            return List.of();
        }
        final List<Renderable> renderables = new ArrayList<>();
        renderables.add(background);
        renderables.addAll(elements);
        hudElements.forEach(hudElement -> renderables.addAll(hudElement.getSubRenderables()));
        renderables.addAll(uiSelector.getSubRenderables());
        return renderables;
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        background.update(dt);
        elements.forEach(button -> button.update(dt, mouseX, mouseY));
        hudElements.forEach(button -> button.update(dt));
        uiSelector.update(dt, mouseX, mouseY);
    }

    public void activate(final boolean active) {
        this.active = active;
    }

    public boolean isActive() {
        return active;
    }

    public void mousePressed(int button, float mouseX, float mouseY) {
        elements.forEach(button1 -> button1.mousePressed(button, mouseX, mouseY));
        uiSelector.mousePressed(button, mouseX, mouseY);
    }
}
