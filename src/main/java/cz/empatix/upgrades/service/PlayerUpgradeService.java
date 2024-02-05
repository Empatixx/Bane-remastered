package cz.empatix.upgrades.service;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.postprocessing.domain.GaussianBlur;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;

public class PlayerUpgradeService {
    private final List<PlayerUpgrade> playerUpgrades;
    private final List<CommonImage> upgradeImages;
    private boolean isActive;
    public PlayerUpgradeService(){
        final CommonImage utilityImage = new CommonImage("ingame/utility_icon.tga", new Vector2f(), 4f, 2, RenderScene.OVERLAYS);
        utilityImage.setPosition(364, 50);
        final CommonImage offenseImage = new CommonImage("ingame/offense_icon.tga", new Vector2f(), 4f, 2, RenderScene.OVERLAYS);
        offenseImage.setPosition(464, 50);
        final CommonImage defenseImage = new CommonImage("ingame/defense_icon.tga", new Vector2f(), 4f, 2, RenderScene.OVERLAYS);
        defenseImage.setPosition(564, 50);

        final CommonImage connect = new CommonImage("ingame/talent_connect.tga", new Vector2f(388, 179), 2f, 3, RenderScene.OVERLAYS);

        final PlayerUpgrade utilityUpgrade = new PlayerUpgrade("ingame/talentframe1.tga");
        final PlayerUpgrade offenseUpgrade = new PlayerUpgrade("ingame/talentframe1.tga");
        final PlayerUpgrade defenseUpgrade = new PlayerUpgrade("ingame/talentframe1.tga");

        utilityUpgrade.setPosition(364, 125);
        offenseUpgrade.setPosition(464, 125);
        defenseUpgrade.setPosition(564, 125);

        final PlayerUpgrade healthUpgrade = new PlayerUpgrade("ingame/talentframe1_locked.tga");
        final PlayerUpgrade damageUpgrade = new PlayerUpgrade("ingame/talentframe1_locked.tga");
        final PlayerUpgrade speedUpgrade = new PlayerUpgrade("ingame/talentframe1_locked.tga");

        healthUpgrade.setPosition(364, 200);
        damageUpgrade.setPosition(464, 200);
        speedUpgrade.setPosition(564, 200);

        final CommonImage connect2 = new CommonImage("ingame/talent_connect_locked.tga", new Vector2f(388, 254), 2f, 3, RenderScene.OVERLAYS);


        final PlayerUpgrade ammoUpgrade = new PlayerUpgrade("ingame/talentframe2_locked.tga");
        final PlayerUpgrade armorUpgrade = new PlayerUpgrade("ingame/talentframe2_locked.tga");
        final PlayerUpgrade discountUpgrade = new PlayerUpgrade("ingame/talentframe2_locked.tga");

        ammoUpgrade.setPosition(364, 275);
        armorUpgrade.setPosition(464, 275);
        discountUpgrade.setPosition(564, 275);

        final CommonImage connect3 = new CommonImage("ingame/talent_connect_locked.tga", new Vector2f(388, 330), 2f, 3, RenderScene.OVERLAYS);


        final PlayerUpgrade armorRegenUpgrade = new PlayerUpgrade("ingame/talentframe3_locked.tga");
        final PlayerUpgrade damageReductionUpgrade = new PlayerUpgrade("ingame/talentframe3_locked.tga");
        final PlayerUpgrade criticalChanceUpgrade = new PlayerUpgrade("ingame/talentframe3_locked.tga");

        armorRegenUpgrade.setPosition(364, 350);
        damageReductionUpgrade.setPosition(464, 350);
        criticalChanceUpgrade.setPosition(564, 350);

        final CommonImage connect4 = new CommonImage("ingame/talent_connect_locked.tga", new Vector2f(388, 404), 2f, 3, RenderScene.OVERLAYS);


        final PlayerUpgrade slotUpgrade = new PlayerUpgrade("ingame/talentframe2_locked.tga");
        final PlayerUpgrade healthUpgrade2 = new PlayerUpgrade("ingame/talentframe2_locked.tga");
        final PlayerUpgrade damageUpgrade2 = new PlayerUpgrade("ingame/talentframe2_locked.tga");

        slotUpgrade.setPosition(364, 425);
        healthUpgrade2.setPosition(464, 425);
        damageUpgrade2.setPosition(564, 425);

        playerUpgrades = List.of(utilityUpgrade, offenseUpgrade, defenseUpgrade, healthUpgrade,
                damageUpgrade, speedUpgrade, ammoUpgrade, armorUpgrade, discountUpgrade, armorRegenUpgrade,
                damageReductionUpgrade, criticalChanceUpgrade, slotUpgrade, healthUpgrade2, damageUpgrade2);
        upgradeImages = List.of(utilityImage, offenseImage, defenseImage, connect, connect2, connect3, connect4);
        isActive = false;
    }
    public List<Renderable> toRender() {
        final List<Renderable> renderables = new ArrayList<>();
        renderables.addAll(playerUpgrades);
        renderables.addAll(upgradeImages);
        return renderables;
    }
    public boolean isActive() {
        return isActive;
    }
    public void setActive(final boolean active) {
        isActive = active;
        GaussianBlur.setEnabled(active);
    }
}
