package cz.empatix.upgrades.service;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.postprocessing.domain.GaussianBlur;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.QualityLevel;
import cz.empatix.upgrades.domain.TreeType;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

public class PlayerUpgradeService {
    private static final Logger LOGGER = LoggerFactory.getLogger(PlayerUpgradeService.class);
    private final List<UpgradeTree> upgradeTrees;
    private PlayerUpgrade selectedUpgrade;
    private final CommonImage upgradeBackground;
    private boolean isActive;

    public PlayerUpgradeService() {
        upgradeBackground = new CommonImage("ingame/pausebg.tga", new Vector2f(675, 150), 3.0f, 2, RenderScene.OVERLAYS);

        final PlayerUpgrade utilityUpgrade = new PlayerUpgrade("ingame/talents/boot.tga", QualityLevel.COMMON, false);
        final PlayerUpgrade offenseUpgrade = new PlayerUpgrade("ingame/talents/bloodtear.tga", QualityLevel.COMMON, false);
        final PlayerUpgrade defenseUpgrade = new PlayerUpgrade("ingame/talents/health_cross.tga", QualityLevel.COMMON, false);

        utilityUpgrade.setTitle("Swiftwind Steps");
        offenseUpgrade.setTitle("Sharpened Bullets");
        defenseUpgrade.setTitle("Hero's Heart");

        utilityUpgrade.setDescription("Increases movement speed\n by 10%");
        offenseUpgrade.setDescription("Increases all damage\n by 20%");
        defenseUpgrade.setDescription("Increases initial health\n by 20%");

        utilityUpgrade.setPosition(364, 125);
        offenseUpgrade.setPosition(464, 125);
        defenseUpgrade.setPosition(564, 125);

        final PlayerUpgrade healthUpgrade = new PlayerUpgrade("ingame/talents/armor.tga", QualityLevel.COMMON, false);
        final PlayerUpgrade damageUpgrade = new PlayerUpgrade("ingame/talents/ammo.tga", QualityLevel.COMMON, false);
        final PlayerUpgrade speedUpgrade = new PlayerUpgrade("ingame/talents/discount.tga", QualityLevel.COMMON, false);

        speedUpgrade.setTitle("Discount");
        damageUpgrade.setTitle("Additional Ammo");
        healthUpgrade.setTitle("Improved Armor");

        speedUpgrade.setDescription("The shop prices will\n have a chance to\n be reduced");
        damageUpgrade.setDescription("Increases ammo capacity\n by 10%");
        healthUpgrade.setDescription("Increases initial armor\n by 33%");

        speedUpgrade.setPosition(364, 199);
        damageUpgrade.setPosition(464, 199);
        healthUpgrade.setPosition(564, 199);

        final PlayerUpgrade discountUpgrade = new PlayerUpgrade("ingame/talents/trap.tga", QualityLevel.RARE, false);
        final PlayerUpgrade ammoUpgrade = new PlayerUpgrade("ingame/talents/criticalhit.tga", QualityLevel.RARE, false);
        final PlayerUpgrade armorUpgrade = new PlayerUpgrade("ingame/talents/armorregen.tga", QualityLevel.RARE, false);

        discountUpgrade.setTitle("Trap Master");
        ammoUpgrade.setTitle("Critical Hit");
        armorUpgrade.setTitle("Armor Regeneration");

        discountUpgrade.setDescription("All traps have a 30% chance\n to be immobilized");
        ammoUpgrade.setDescription("Your attacks have a 10%\n chance to deal double\n damage");
        armorUpgrade.setDescription("Restores armor after\n successfully\n clearing a room");

        discountUpgrade.setPosition(364, 273);
        ammoUpgrade.setPosition(464, 273);
        armorUpgrade.setPosition(564, 273);

        final PlayerUpgrade armorRegenUpgrade = new PlayerUpgrade("ingame/talents/health_cross2.tga", QualityLevel.EPIC, true);
        final PlayerUpgrade criticalChanceUpgrade = new PlayerUpgrade("ingame/talents/bloodtear2.tga", QualityLevel.EPIC, true);
        final PlayerUpgrade damageReductionUpgrade = new PlayerUpgrade("ingame/talents/extragun.tga", QualityLevel.EPIC, true);

        armorRegenUpgrade.setTitle("Titan's Heart");
        criticalChanceUpgrade.setTitle("Dangerous Bullets");
        damageReductionUpgrade.setTitle("Additional Gun");

        armorRegenUpgrade.setDescription("Increases initial health\n by 25%");
        criticalChanceUpgrade.setDescription("Increases all damage\n by 25%");
        damageReductionUpgrade.setDescription("Adds an additional slot\n for a weapon");

        armorRegenUpgrade.setPosition(364, 347);
        criticalChanceUpgrade.setPosition(464, 347);
        damageReductionUpgrade.setPosition(564, 347);

        final PlayerUpgrade slotUpgrade = new PlayerUpgrade("ingame/talents/teleportation.tga", QualityLevel.COMMON, true);
        final PlayerUpgrade healthUpgrade2 = new PlayerUpgrade("ingame/talents/kevlar.tga", QualityLevel.COMMON, true);
        final PlayerUpgrade damageUpgrade2 = new PlayerUpgrade("ingame/talents/pet.tga", QualityLevel.COMMON, true);

        slotUpgrade.setTitle("Teleportation");
        healthUpgrade2.setTitle("Kevlar");
        damageUpgrade2.setTitle("Pet");

        slotUpgrade.setDescription("Teleports you to\n choosen and discovered\n room from the minimap");
        healthUpgrade2.setDescription("Reduces damage taken\n by 10%");
        damageUpgrade2.setDescription("Unlocks a pet that will\n help you in battle\n");

        slotUpgrade.setPosition(364, 421);
        damageUpgrade2.setPosition(464, 421);
        healthUpgrade2.setPosition(564, 421);

        isActive = false;

        final UpgradeTree utilityTree = new UpgradeTree(List.of(utilityUpgrade, speedUpgrade, discountUpgrade, damageReductionUpgrade, slotUpgrade), TreeType.UTILITY);
        utilityTree.setPosition(364, 125);

        final UpgradeTree offenseTree = new UpgradeTree(List.of(offenseUpgrade, damageUpgrade, ammoUpgrade, criticalChanceUpgrade, damageUpgrade2), TreeType.OFFENSIVE);
        offenseTree.setPosition(464, 125);

        final UpgradeTree defenseTree = new UpgradeTree(List.of(defenseUpgrade, healthUpgrade, armorUpgrade, armorRegenUpgrade, healthUpgrade2), TreeType.DEFENSIVE);
        defenseTree.setPosition(564, 125);

        upgradeTrees = List.of(utilityTree, offenseTree, defenseTree);
    }

    public List<Renderable> toRender() {
        final List<Renderable> renderables = upgradeTrees.stream()
                .flatMap(tree -> tree.getSubRenderables().stream())
                .collect(Collectors.toList());
        if (selectedUpgrade != null) {
            renderables.add(upgradeBackground);
            renderables.add(selectedUpgrade);
            renderables.addAll(selectedUpgrade.getCurrentRenderables());
        }
        return renderables;
    }

    public void mousePressed(final float mouseX, final float mouseY) {
        if (isActive) {
            final List<PlayerUpgrade> pressedUpgrades = upgradeTrees.stream()
                    .map(tree -> tree.getPressedUpgrade(mouseX, mouseY))
                    .filter(Objects::nonNull)
                    .toList();
            if (pressedUpgrades.size() != 1) {
                LOGGER.warn("Pressed multiple upgrades from trees: {}", pressedUpgrades.size());
            }
            final PlayerUpgrade currentUpgrade = pressedUpgrades.isEmpty() ? null : pressedUpgrades.get(0);
            selectedUpgrade = pressedUpgrades.isEmpty() ? null : currentUpgrade.deepCopy();
            if (selectedUpgrade != null) {
                selectedUpgrade.setPosition(675 + upgradeBackground.getWidth() / 2f - selectedUpgrade.getWidth() / 2f, 90);
                selectedUpgrade.setTitle(currentUpgrade.getTitle());
                selectedUpgrade.setDescription(currentUpgrade.getDescription());
            }
        }
    }

    public boolean isActive() {
        return isActive;
    }

    public void setActive(final boolean active) {
        isActive = active;
        GaussianBlur.setEnabled(active);
    }

    public void update(final double dt,final float mouseX, final float mouseY) {
        if (isActive) {
            upgradeTrees.forEach(tree -> tree.update(dt,mouseX, mouseY));
        }
    }
}
