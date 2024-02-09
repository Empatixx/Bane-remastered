package cz.empatix.upgrades.service;

import cz.empatix.hud.service.CommonBackground;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.postprocessing.domain.GaussianBlur;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.QualityLevel;
import cz.empatix.upgrades.domain.TreeType;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class PlayerUpgradeService {
    private final List<UpgradeTree> upgradeTrees;
    private final CommonImage upgradeBackground;
    private boolean isActive;
    public PlayerUpgradeService(){
        upgradeBackground = new CommonImage("ingame/pausebg.tga", new Vector2f(675,150), 3, 2, RenderScene.OVERLAYS);

        final PlayerUpgrade utilityUpgrade = new PlayerUpgrade(QualityLevel.COMMON, false);
        final PlayerUpgrade offenseUpgrade = new PlayerUpgrade(QualityLevel.COMMON, false);
        final PlayerUpgrade defenseUpgrade = new PlayerUpgrade(QualityLevel.COMMON, false);

        utilityUpgrade.setPosition(364, 125);
        offenseUpgrade.setPosition(464, 125);
        defenseUpgrade.setPosition(564, 125);

        final PlayerUpgrade healthUpgrade = new PlayerUpgrade(QualityLevel.COMMON, false);
        final PlayerUpgrade damageUpgrade = new PlayerUpgrade(QualityLevel.COMMON, false);
        final PlayerUpgrade speedUpgrade = new PlayerUpgrade(QualityLevel.COMMON, false);

        speedUpgrade.setPosition(364, 199);
        damageUpgrade.setPosition(464, 199);
        healthUpgrade.setPosition(564, 199);


        final PlayerUpgrade discountUpgrade = new PlayerUpgrade(QualityLevel.RARE, false);
        final PlayerUpgrade ammoUpgrade = new PlayerUpgrade(QualityLevel.RARE, false);
        final PlayerUpgrade armorUpgrade = new PlayerUpgrade(QualityLevel.RARE, false);

        discountUpgrade.setPosition(364, 273);
        ammoUpgrade.setPosition(464, 273);
        armorUpgrade.setPosition(564, 273);


        final PlayerUpgrade armorRegenUpgrade = new PlayerUpgrade(QualityLevel.EPIC, true);
        final PlayerUpgrade damageReductionUpgrade = new PlayerUpgrade(QualityLevel.EPIC, true);
        final PlayerUpgrade criticalChanceUpgrade = new PlayerUpgrade(QualityLevel.EPIC, true);

        armorRegenUpgrade.setPosition(364, 347);
        criticalChanceUpgrade.setPosition(464, 347);
        damageReductionUpgrade.setPosition(564, 347);

        final PlayerUpgrade slotUpgrade = new PlayerUpgrade(QualityLevel.COMMON, true);
        final PlayerUpgrade healthUpgrade2 = new PlayerUpgrade(QualityLevel.COMMON, true);
        final PlayerUpgrade damageUpgrade2 = new PlayerUpgrade(QualityLevel.COMMON, true);

        slotUpgrade.setPosition(364, 421);
        damageUpgrade2.setPosition(464, 421);
        healthUpgrade2.setPosition(564, 421);

        isActive = false;

        final UpgradeTree utilityTree = new UpgradeTree(List.of(utilityUpgrade, speedUpgrade, discountUpgrade, armorRegenUpgrade, slotUpgrade), TreeType.UTILITY);
        utilityTree.setPosition(364, 125);

        final UpgradeTree offenseTree = new UpgradeTree(List.of(offenseUpgrade, damageUpgrade, ammoUpgrade, criticalChanceUpgrade, damageUpgrade2), TreeType.OFFENSIVE);
        offenseTree.setPosition(464, 125);

        final UpgradeTree defenseTree = new UpgradeTree(List.of(defenseUpgrade, healthUpgrade, armorUpgrade, damageReductionUpgrade, healthUpgrade2), TreeType.DEFENSIVE);
        defenseTree.setPosition(564, 125);

        upgradeTrees = List.of(utilityTree, offenseTree, defenseTree);
    }
    public List<Renderable> toRender() {
        final List<Renderable> trees = upgradeTrees.stream()
                .flatMap(tree -> tree.getSubRenderables().stream())
                .collect(Collectors.toList());
        trees.add(upgradeBackground);
        return trees;
    }
    public boolean isActive() {
        return isActive;
    }
    public void setActive(final boolean active) {
        isActive = active;
        GaussianBlur.setEnabled(active);
    }
}
