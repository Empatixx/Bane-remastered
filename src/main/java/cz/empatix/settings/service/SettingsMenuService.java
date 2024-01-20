package cz.empatix.settings.service;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.hud.service.CommonBackground;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.settings.domain.GameSettings;
import cz.empatix.ui.*;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3i;

import java.util.ArrayList;
import java.util.List;

public class SettingsMenuService {
    private final static Vector3i TEXT_COLOR = new Vector3i(200, 92, 18);
    private final SettingsService settingsService = SettingsService.getInstance();
    private final CommonBackground background;
    private final List<UIElement> commonElements;
    private final List<UIElement> videoElements;
    private final List<UIElement> audioElements;
    private final List<UIElement> controlsElements;
    private final List<HUDElement> videoHudElements;
    private final List<HUDElement> audioHudElements;
    private final List<HUDElement> controlsHudElements;
    private final UISelector<SettingsType> uiSelector;
    private final SlidingListText<Vector2i> resolutionText;
    private boolean active;

    public SettingsMenuService() {
        background = new CommonBackground("menu/settings_background.tga", 480, 320);

        final GameSettings gameSettings = settingsService.getSettings();

        final UISelector<SettingsType> selector = new UISelector<>();
        final StickyHoverButton video = new StickyHoverButton("Video", "menu/settings_menubar.tga", 30, 16, new Vector2f(325, 160), 5f, FontSize.FONT_SIZE_42);
        final StickyHoverButton audio = new StickyHoverButton("Audio", "menu/settings_menubar.tga", 30, 16, new Vector2f(480, 160), 5, FontSize.FONT_SIZE_42);
        final StickyHoverButton controls = new StickyHoverButton("Controls", "menu/settings_menubar.tga", 30, 16, new Vector2f(635, 160), 5, FontSize.FONT_SIZE_42);
        selector.addElement(video, SettingsType.VIDEO);
        selector.addElement(audio, SettingsType.AUDIO);
        selector.addElement(controls, SettingsType.CONTROLS);

        final Checkbox vsyncCheckbox = new Checkbox();
        vsyncCheckbox.setPosition(470, 225);
        vsyncCheckbox.check(gameSettings.isVsync());
        vsyncCheckbox.onCheck(settingsService::setVsync);

        final CommonText vsyncLabel = new CommonText("V-sync:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        vsyncLabel.setPosition(275, 225);
        vsyncLabel.setCenterY(vsyncCheckbox.getPosition().y(), vsyncCheckbox.getPosition().y() + vsyncCheckbox.getHeight());
        vsyncLabel.setColorRGB(TEXT_COLOR);

        final Checkbox fullscreenCheckbox = new Checkbox();
        fullscreenCheckbox.setPosition(470, 275);
        fullscreenCheckbox.check(gameSettings.isFullscreen());
        fullscreenCheckbox.onCheck(settingsService::setFullscreen);

        final CommonText fullscreenLabel = new CommonText("Fullscreen:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        fullscreenLabel.setPosition(275, 275);
        fullscreenLabel.setCenterY(fullscreenCheckbox.getPosition().y(), fullscreenCheckbox.getPosition().y() + fullscreenCheckbox.getHeight());
        fullscreenLabel.setColorRGB(TEXT_COLOR);

        final CommonText resolutionLabel = new CommonText("Resolution:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        resolutionLabel.setPosition(275, 341);
        resolutionLabel.setColorRGB(TEXT_COLOR);

        resolutionText = new SlidingListText<>(new SlidingList<>(settingsService.getResolutions()), resolution -> resolution.x + "x" + resolution.y);
        resolutionText.setColorRGB(TEXT_COLOR);
        resolutionText.onChange(settingsService::setResolution);
        resolutionText.slideToValue(new Vector2i(gameSettings.getResolutionWidth(), gameSettings.getResolutionHeight()));

        final CommonButton nextResolution = new CommonButton("", "menu/button_arrow_rotated.tga", 16, 16, new Vector2f(560, 335), 2);
        nextResolution.onClick(resolutionText::next);

        final CommonButton previousResolution = new CommonButton("", "menu/button_arrow.tga", 16, 16, new Vector2f(410, 335), 2);
        previousResolution.onClick(resolutionText::previous);
        resolutionText.center(410, 560, 340);

        final Slider overallSlider = new Slider();
        overallSlider.setPosition(425, 225);
        overallSlider.setTextPosition(365, 225);
        overallSlider.setCenterTextY(overallSlider.getPosition().y(), overallSlider.getPosition().y() + overallSlider.getHeight());
        overallSlider.setTextRGB(new Vector3i(223, 60, 28));
        overallSlider.onChange(settingsService::setOverallVolume);
        overallSlider.setValue(gameSettings.getOverallVolume());

        final Slider musicSlider = new Slider();
        musicSlider.setPosition(425, 275);
        musicSlider.setTextPosition(365, 275);
        musicSlider.setCenterTextY(musicSlider.getPosition().y(), musicSlider.getPosition().y() + musicSlider.getHeight());
        musicSlider.setTextRGB(new Vector3i(223, 60, 28));
        musicSlider.onChange(settingsService::setMusicVolume);
        musicSlider.setValue(gameSettings.getMusicVolume());

        final Slider effectsSlider = new Slider();
        effectsSlider.setPosition(425, 325);
        effectsSlider.setTextPosition(365, 325);
        effectsSlider.setCenterTextY(effectsSlider.getPosition().y(), effectsSlider.getPosition().y() + effectsSlider.getHeight());
        effectsSlider.setTextRGB(new Vector3i(223, 60, 28));
        effectsSlider.onChange(settingsService::setEffectsVolume);
        effectsSlider.setValue(gameSettings.getEffectsVolume());

        final CommonText overallLabel = new CommonText("Overall:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        overallLabel.setPosition(275, 225);
        overallLabel.setCenterY(overallSlider.getPosition().y(), overallSlider.getPosition().y() + overallSlider.getHeight());
        overallLabel.setColorRGB(TEXT_COLOR);

        final CommonText musicLabel = new CommonText("Music:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        musicLabel.setPosition(275, 275);
        musicLabel.setCenterY(musicSlider.getPosition().y(), musicSlider.getPosition().y() + musicSlider.getHeight());
        musicLabel.setColorRGB(TEXT_COLOR);

        final CommonText effectsLabel = new CommonText("Effects:", "PixelFJVerdana12pt.ttf", new Vector2f(), vsyncCheckbox.getZIndex());
        effectsLabel.setPosition(275, 325);
        effectsLabel.setCenterY(effectsSlider.getPosition().y(), effectsSlider.getPosition().y() + effectsSlider.getHeight());
        effectsLabel.setColorRGB(TEXT_COLOR);

        final CommonButton applyButton = new CommonButton("Apply", new Vector2f(800, 160), 1.5f, FontSize.FONTSIZE_32);
        applyButton.onClick(settingsService::apply);

        final CommonButton revertButton = new CommonButton("Revert", new Vector2f(800, 240), 1.5f, FontSize.FONTSIZE_32);
        revertButton.onClick(() -> {
            settingsService.revert();
            resolutionText.slideToValue(new Vector2i(gameSettings.getResolutionWidth(), gameSettings.getResolutionHeight()));
            vsyncCheckbox.check(gameSettings.isVsync());
            fullscreenCheckbox.check(gameSettings.isFullscreen());
        });

        final CommonButton exitButton = new CommonButton("Exit", new Vector2f(800, 320), 1.5f, FontSize.FONTSIZE_32);
        exitButton.onClick(() -> activate(false));

        uiSelector = selector;
        commonElements = List.of(exitButton, revertButton, applyButton);
        videoElements = List.of(vsyncCheckbox, fullscreenCheckbox, nextResolution, previousResolution);
        audioElements = List.of(overallSlider, musicSlider, effectsSlider);
        controlsElements = List.of();

        videoHudElements = List.of(vsyncLabel, fullscreenLabel, resolutionLabel);
        audioHudElements = List.of(overallLabel, musicLabel, effectsLabel);
        controlsHudElements = List.of();
    }

    public List<Renderable> toRender(final double dt) {
        if (!active) {
            return List.of();
        }
        return switch (uiSelector.getSelected()) {
            case VIDEO -> {
                final List<Renderable> renderables = new ArrayList<>();
                renderables.add(background);
                renderables.addAll(commonElements);
                renderables.addAll(videoElements);
                videoHudElements.forEach(hudElement -> renderables.addAll(hudElement.getSubRenderables()));
                renderables.addAll(resolutionText.getSubRenderables());
                renderables.addAll(uiSelector.getSubRenderables());
                yield renderables;
            }
            case AUDIO-> {
                final List<Renderable> renderables = new ArrayList<>();
                renderables.add(background);
                renderables.addAll(commonElements);
                renderables.addAll(audioElements);
                audioHudElements.forEach(hudElement -> renderables.addAll(hudElement.getSubRenderables()));
                renderables.addAll(uiSelector.getSubRenderables());
                yield renderables;
            }
            case CONTROLS -> {
                final List<Renderable> renderables = new ArrayList<>();
                renderables.add(background);
                renderables.addAll(commonElements);
                renderables.addAll(controlsElements);
                controlsHudElements.forEach(hudElement -> renderables.addAll(hudElement.getSubRenderables()));
                renderables.addAll(uiSelector.getSubRenderables());
                yield renderables;
            }
        };
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        background.update(dt);
        commonElements.forEach(button -> button.update(dt, mouseX, mouseY));
        videoElements.forEach(button -> button.update(dt, mouseX, mouseY));
        audioElements.forEach(button -> button.update(dt, mouseX, mouseY));
        controlsElements.forEach(button -> button.update(dt, mouseX, mouseY));
        videoHudElements.forEach(button -> button.update(dt));
        uiSelector.update(dt, mouseX, mouseY);
    }

    public void activate(final boolean active) {
        this.active = active;
    }

    public boolean isActive() {
        return active;
    }

    public void mousePressed(int btn, float mouseX, float mouseY) {
        switch (uiSelector.getSelected()) {
            case VIDEO -> videoElements.forEach(button -> button.mousePressed(btn, mouseX, mouseY));
            case AUDIO -> audioElements.forEach(button -> button.mousePressed(btn, mouseX, mouseY));
            case CONTROLS -> controlsElements.forEach(button -> button.mousePressed(btn, mouseX, mouseY));
        }
        commonElements.forEach(button -> button.mousePressed(btn, mouseX, mouseY));
        uiSelector.mousePressed(btn, mouseX, mouseY);
    }
    public void mouseReleased(int btn, float mouseX, float mouseY) {
        switch (uiSelector.getSelected()) {
            case VIDEO -> videoElements.forEach(button -> button.mouseReleased(btn, mouseX, mouseY));
            case AUDIO -> audioElements.forEach(button -> button.mouseReleased(btn, mouseX, mouseY));
            case CONTROLS -> controlsElements.forEach(button -> button.mouseReleased(btn, mouseX, mouseY));
        }
        commonElements.forEach(button -> button.mouseReleased(btn, mouseX, mouseY));
        uiSelector.mouseReleased(btn, mouseX, mouseY);
    }
}
