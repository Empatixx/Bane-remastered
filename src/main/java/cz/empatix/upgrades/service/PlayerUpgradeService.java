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

        utilityUpgrade.setPosition(364, 125);
        offenseUpgrade.setPosition(464, 125);
        defenseUpgrade.setPosition(564, 125);

        final PlayerUpgrade healthUpgrade = new PlayerUpgrade("ingame/talents/armor.tga", QualityLevel.COMMON, false);
        final PlayerUpgrade damageUpgrade = new PlayerUpgrade("ingame/talents/ammo.tga", QualityLevel.COMMON, false);
        final PlayerUpgrade speedUpgrade = new PlayerUpgrade("ingame/talents/discount.tga", QualityLevel.COMMON, false);

        speedUpgrade.setPosition(364, 199);
        damageUpgrade.setPosition(464, 199);
        healthUpgrade.setPosition(564, 199);


        final PlayerUpgrade discountUpgrade = new PlayerUpgrade("ingame/talents/kevlar.tga", QualityLevel.RARE, false);
        final PlayerUpgrade ammoUpgrade = new PlayerUpgrade("ingame/talents/criticalhit.tga", QualityLevel.RARE, false);
        final PlayerUpgrade armorUpgrade = new PlayerUpgrade("ingame/talents/armorregen.tga", QualityLevel.RARE, false);

        discountUpgrade.setPosition(364, 273);
        ammoUpgrade.setPosition(464, 273);
        armorUpgrade.setPosition(564, 273);


        final PlayerUpgrade armorRegenUpgrade = new PlayerUpgrade("ingame/talents/health_cross2.tga", QualityLevel.EPIC, true);
        final PlayerUpgrade criticalChanceUpgrade = new PlayerUpgrade("ingame/talents/bloodtear2.tga", QualityLevel.EPIC, true);
        final PlayerUpgrade damageReductionUpgrade = new PlayerUpgrade("ingame/talents/extragun.tga", QualityLevel.EPIC, true);

        armorRegenUpgrade.setPosition(364, 347);
        criticalChanceUpgrade.setPosition(464, 347);
        damageReductionUpgrade.setPosition(564, 347);

        final PlayerUpgrade slotUpgrade = new PlayerUpgrade("ingame/talents/boot.tga", QualityLevel.COMMON, true);
        final PlayerUpgrade healthUpgrade2 = new PlayerUpgrade("ingame/talents/boot.tga", QualityLevel.COMMON, true);
        final PlayerUpgrade damageUpgrade2 = new PlayerUpgrade("ingame/talents/boot.tga", QualityLevel.COMMON, true);

        slotUpgrade.setPosition(364, 421);
        damageUpgrade2.setPosition(464, 421);
        healthUpgrade2.setPosition(564, 421);

        isActive = false;

        final UpgradeTree utilityTree = new UpgradeTree(List.of(utilityUpgrade, speedUpgrade, discountUpgrade,damageReductionUpgrade, slotUpgrade), TreeType.UTILITY);
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
            selectedUpgrade = pressedUpgrades.isEmpty() ? null : pressedUpgrades.get(0).deepCopy();
            if (selectedUpgrade != null) {
                selectedUpgrade.setPosition(675 + upgradeBackground.getWidth() / 2f - selectedUpgrade.getWidth() / 2f, 90);
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
}
