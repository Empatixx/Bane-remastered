package cz.empatix.upgrades.service;

import cz.empatix.postprocessing.domain.GaussianBlur;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;

public class PlayerUpgradeService {
    private static final Logger LOGGER = LoggerFactory.getLogger(PlayerUpgradeService.class);
    private final List<UpgradeTree> upgradeTrees;
    private final CurrentUpgradeMenu currentUpgradeMenu;
    private final FileUpgradeService fileUpgradeService;
    private boolean isActive;

    public PlayerUpgradeService() {
        currentUpgradeMenu = new CurrentUpgradeMenu();
        fileUpgradeService = new FileUpgradeService();

        final UpgradeTree utilityTree = UpgradeTreeFactory.createUtilityTree();
        final UpgradeTree offenseTree = UpgradeTreeFactory.createOffenseTree();
        final UpgradeTree defenseTree = UpgradeTreeFactory.createDefenseTree();

        final Map<PlayerUpgradeName, Boolean> boughtUpgrades = fileUpgradeService.load();
        utilityTree.setBoughtUpgrades(boughtUpgrades);

        upgradeTrees = List.of(utilityTree, offenseTree, defenseTree);
    }

    public List<Renderable> toRender() {
        final List<Renderable> renderables = upgradeTrees.stream()
                .flatMap(tree -> tree.getSubRenderables().stream())
                .collect(Collectors.toList());
        renderables.addAll(currentUpgradeMenu.toRender());
        return renderables;
    }

    public void mousePressed(final int button, final float mouseX, final float mouseY) {
        if (isActive) {
            final List<PlayerUpgrade> pressedUpgrades = upgradeTrees.stream()
                    .map(tree -> tree.getPressedUpgrade(mouseX, mouseY))
                    .filter(Objects::nonNull)
                    .toList();
            if (pressedUpgrades.size() != 1) {
                LOGGER.warn("Pressed multiple upgrades from trees: {}", pressedUpgrades.size());
            }
            final boolean eventOccured = currentUpgradeMenu.mousePressed(button, mouseX, mouseY);
            if (eventOccured) {
                return;
            }
            final PlayerUpgrade currentUpgrade = pressedUpgrades.isEmpty() ? null : pressedUpgrades.get(0);
            currentUpgradeMenu.setCurrent(currentUpgrade);
        }
    }

    public boolean isActive() {
        return isActive;
    }

    public void setActive(final boolean active) {
        isActive = active;
        GaussianBlur.setEnabled(active);
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        if (isActive) {
            currentUpgradeMenu.update(dt, mouseX, mouseY);
            upgradeTrees.forEach(tree -> tree.update(dt));
        }
    }

    public void save() {
        final Map<PlayerUpgradeName, Boolean> boughtUpgrades = upgradeTrees.stream()
                .flatMap(tree -> tree.getUpgrades().stream())
                .collect(Collectors.toMap(PlayerUpgrade::getName, PlayerUpgrade::isBought));
        fileUpgradeService.save(boughtUpgrades);
    }
}
