package cz.empatix.guns.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.utils.PositionUtils;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.guns.domain.*;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.items.domain.MovableItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.stream.Stream;

public class GunService {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final static Vector2f HUD_SLOT_POSITION = new Vector2f(715, 450);
    private final static float HUD_SLOT_SCALE = 1.5f;
    public final static int HUD_SLOT_CENTER_X;
    public final static int HUD_SLOT_CENTER_Y;
    private static final long ITEM_DROP_FORCE = 15_000;

    static {
        final TextureAtlas.Entry hudSlotTexture = AssetsService.getTexture("ingame/weaponslot.tga");

        HUD_SLOT_CENTER_X = PositionUtils.getCenterXInRange((int) (HUD_SLOT_POSITION.x()), (int) (HUD_SLOT_POSITION.x() + hudSlotTexture.width() * HUD_SLOT_SCALE));
        HUD_SLOT_CENTER_Y = PositionUtils.getCenterYInRange((int) (HUD_SLOT_POSITION.y()), (int) (HUD_SLOT_POSITION.y() + hudSlotTexture.height() * HUD_SLOT_SCALE));
    }

    private final static Logger LOGGER = LoggerFactory.getLogger(GunService.class);
    private final CommonImage weaponSlotHud;
    private final List<Gun> guns;
    private final GunSlots slots;
    private final ItemService itemService;
    private final StatisticsService statisticsService;

    private float shooterX, shooterY, targetX, targetY;

    public GunService(final ItemService itemService,
                      final StatisticsService statisticsService) {
        this.statisticsService = statisticsService;
        this.itemService = itemService;
        guns = List.of(
                new Pistol(),
                new Shotgun(),
                new Luger(),
                new Revolver(),
                new M4(),
                new Uzi(),
                new Thompson()
        );
        {
            slots = new GunSlots();
            slots.addGun(guns.get(4));
            slots.addGun(guns.get(5));
        }
        weaponSlotHud = new CommonImage(AssetsService.getTexture("ingame/weaponslot.tga"), HUD_SLOT_POSITION, HUD_SLOT_SCALE);
    }

    public void update(final double dt) {
        guns.forEach(gun -> gun.update(dt));
        try{
            if(CursorService.getType() == CursorType.DEFAULT){
                return;
            }
            if (slots.getCurrentGun().isReloading()) {
                CursorService.setCursor(CursorType.RELOAD);
            } else {
                CursorService.setCursor(CursorType.CROSSHAIR);
            }
        } catch (IllegalStateException e) {
            LOGGER.debug("Could not set cursor..");
        }
    }

    public void fixedUpdate(final float shooterX, final float shooterY,
                            final float targetX, final float targetY, final double dt) {
        this.shooterX = shooterX;
        this.shooterY = shooterY;
        this.targetX = targetX;
        this.targetY = targetY;
        try {
            final boolean shoot = slots.getCurrentGun().shoot(shooterX, shooterY, targetX, targetY);
            if (shoot) {
                statisticsService.addBulletFired();
            }
        } catch (IllegalStateException e) {
            LOGGER.debug("Could not shoot gun..");
        }
    }

    public void setShooting(final boolean shooting) {
        try {
            slots.getCurrentGun().setShooting(shooting);
        } catch (IllegalStateException e) {
            LOGGER.debug("Could not set shooting..");
        }
    }

    private void reload() {
        setShooting(false);
        try {
            slots.getCurrentGun().reload();
        } catch (IllegalStateException e) {
            LOGGER.warn("Could not reload..");
        }
    }

    public void addGun(final Gun gun) {
        setShooting(false);
        if (slots.hasFullSlots()) {
            dropCurrentGun();
        }
        slots.addGun(gun);
    }

    public List<Renderable> toRender(final double dt) {
        try {
            return Stream.concat(
                    this.guns.stream()
                            .map(Gun::toRender)
                            .flatMap(List::stream),
                    Stream.concat(
                            Stream.of(weaponSlotHud),
                            slots.getCurrentGun().hudToRender().stream()
                    )
            ).toList();
        } catch (Exception e) {
            LOGGER.warn("Could not render gun renderables..");
            return List.of(weaponSlotHud);
        }
    }

    public void keyPressed(final int button) {
        if (button >= GLFW.GLFW_KEY_1 && button <= GLFW.GLFW_KEY_9) {
            setShooting(false);
            slots.switchSlot(button - GLFW.GLFW_KEY_1);
        }
        switch (button) {
            case GLFW.GLFW_KEY_R -> reload();
            case GLFW.GLFW_KEY_Q -> tryDropCurrentGun();
        }
    }
    public Gun getUnusedRandomGun(){
        Gun gun = guns.get(RANDOM.nextInt(guns.size()));
        while(slots.containsGun(gun)){
            gun = guns.get(RANDOM.nextInt(guns.size()));
        }
        gun.addPercentAmmo(100);
        return gun;
    }

    public boolean hasNotFullAmmo(final GunType gunType){
        return slots.hasNotFullAmmo(gunType);
    }

    public void addAmmo(final GunType gunType, final float percent) {
        slots.addAmmo(gunType, percent);
    }

    public void tryDropCurrentGun(){
        try {
            dropCurrentGun();
        } catch (IllegalStateException e) {
            LOGGER.warn("Could not drop current gun..");
        }
    }
    private void dropCurrentGun(){
        final ItemFactory itemFactory = itemService.getItemFactory();
        final Gun drop = slots.dropCurrentGun();

        final MovableItemDrop itemDrop = itemFactory.createMovableWeaponDrop(drop, this::addGun);
        itemDrop.setPosition(shooterX - itemDrop.getWidth()/2f, shooterY - itemDrop.getHeight()/2f);

        final double atan = Math.atan2(targetY - shooterY, targetX - shooterX);
        final float forceX = (float) Math.cos(atan) * ITEM_DROP_FORCE;
        final float forceY = (float) Math.sin(atan) * ITEM_DROP_FORCE;
        itemDrop.applyForce(forceX, forceY);

        itemService.add(itemDrop);
    }
}
