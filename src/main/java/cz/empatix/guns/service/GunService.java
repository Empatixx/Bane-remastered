package cz.empatix.guns.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.guns.domain.*;
import cz.empatix.hud.service.Image;
import cz.empatix.hud.service.Text;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;

public class GunService {
    private final static Logger LOGGER = LoggerFactory.getLogger(GunService.class);
    private final Image weaponSlotHud;
    private final Text ammoCounter;
    private final List<Gun> guns;
    private final GunSlots slots;

    public GunService() {
        guns = List.of(
                new Pistol(),
                new Shotgun(),
                new Luger(),
                new Revolver(),
                new M4(),
                new Uzi(),
                new Thompson()
        );
        slots = new GunSlots();
        slots.addGun(guns.get(2));
        slots.addGun(guns.get(6));
        weaponSlotHud = new Image(AssetsService.getTexture("ingame/weaponslot.tga"), new Vector2f(715, 450), 1.5f);
        ammoCounter = new Text(slots.getCurrentGun().getAmmoCounter(), "PixelFJVerdana12pt.ttf", new Vector2f(885, 515), 10);
    }

    public void update(final double dt){
        guns.forEach(gun -> gun.update(dt));
        try{
            ammoCounter.setText(slots.getCurrentGun().getAmmoCounter());
        } catch (IllegalStateException e){
            ammoCounter.setText("");
            LOGGER.warn("Could not update ammo counter..");
        }
    }

    public void fixedUpdate(final float shooterX, final float shooterY,
                            final float targetX, final float targetY, final double dt) {
        try {
            slots.getCurrentGun().shoot(shooterX, shooterY, targetX, targetY);
        } catch (IllegalStateException e) {
            LOGGER.warn("Could not shoot gun..");

        }
    }

    public void setShooting(final boolean shooting) {
        try{
            slots.getCurrentGun().setShooting(shooting);
        } catch (IllegalStateException e){
            LOGGER.warn("Could not set shooting..");
        }
    }

    private void reload() {
        try{
            slots.getCurrentGun().reload();
        } catch (IllegalStateException e) {
            LOGGER.warn("Could not reload..");
        }
    }

    public void addGun(final Gun gun) {
        if (slots.hasFullSlots()) {
            final Gun drop = slots.dropCurrentGun();
            slots.addGun(gun);
            return;
        }
        slots.addGun(gun);
    }

    public List<Renderable> toRender(final double dt) {
        final List<Renderable> gunsRenderable = guns.stream()
                .flatMap(gun -> gun.toRender().stream())
                .toList();
        final List<Renderable> renderables = new ArrayList<>(gunsRenderable);
        renderables.add(weaponSlotHud);
        renderables.addAll(ammoCounter.getSubRenderables());
        return renderables;
    }

    public void keyPressed(final int button) {
        if(button >= GLFW.GLFW_KEY_1 && button <= GLFW.GLFW_KEY_9){
            slots.switchSlot(button - GLFW.GLFW_KEY_1);
        }
        if (GLFW.GLFW_KEY_R == button) {
            reload();
        }
    }
}
