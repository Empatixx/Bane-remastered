package cz.empatix.settings.service;

import cz.empatix.hud.service.CommonBackground;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.ui.CommonButton;

import java.util.List;

public class SettingsMenuService {
    private final SettingsService settingsService = SettingsService.getInstance();
    private final CommonBackground background = new CommonBackground("menu/settings_background.tga",480,320);
    private final List<CommonButton> buttons = List.of();
    private boolean active;

    public List<Renderable> toRender(final double dt){
        if(!active){
            return List.of();
        }
        return List.of(background);
    }
    public void update(final double dt){
        background.update(dt);
    }
    public void activate(final boolean active){
        this.active = active;
    }

    public boolean isActive() {
        return active;
    }
}
