package cz.empatix.upgrades.service;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.TalentPath;
import cz.empatix.upgrades.domain.TreeType;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;

public class UpgradeTree {
    private final static Logger LOGGER = LoggerFactory.getLogger(UpgradeTree.class);
    private final CommonImage treeIcon;
    private final List<PlayerUpgrade> upgrades;
    private final List<TalentPath> parentConnectors;
    private final TreeType type;

    public UpgradeTree(final List<PlayerUpgrade> upgrades, final TreeType type) {
        this.upgrades = upgrades;
        this.parentConnectors = upgrades.stream()
                .skip(1)
                .limit(upgrades.size() - 1)
                .map(upgrade -> {
                    final TalentPath path = new TalentPath(type, () -> !upgrade.isBought());
                    return path;
                })
                .toList();

        final String texture = switch (type) {
            case OFFENSIVE -> "ingame/offense_icon.tga";
            case DEFENSIVE -> "ingame/defense_icon.tga";
            case UTILITY -> "ingame/utility_icon.tga";
        };
        treeIcon = new CommonImage(texture, new Vector2f(), 4f, 3, RenderScene.OVERLAYS);
        this.type = type;
    }
    public void update(final double dt){
        parentConnectors.forEach(path -> path.update(dt));
    }

    public void setPosition(final float x, final float y) {
        ;
        treeIcon.setPosition(x, y - 75);
        for (int i = 0; i < upgrades.size(); i++) {
            upgrades.get(i).setPosition(x, y + i * 74);
        }
        for (int i = 0; i < parentConnectors.size(); i++) {
            parentConnectors.get(i).setPosition(x + 24, y + (i + 1) * 74 - 20);
        }
    }

    public List<Renderable> getSubRenderables() {
        final List<Renderable> renderables = new ArrayList<>(List.copyOf(upgrades));
        renderables.addAll(parentConnectors);
        renderables.add(treeIcon);
        return renderables;
    }

    public PlayerUpgrade getPressedUpgrade(final float mouseX, final float mouseY) {
        final List<PlayerUpgrade> upgrades = this.upgrades.stream()
                .filter(upgrade -> upgrade.isPressed(mouseX, mouseY))
                .toList();
        if (upgrades.size() > 1) {
            LOGGER.warn("Multiple upgrades pressed at the same time! in type: {}", type);
        }
        return upgrades.isEmpty() ? null : upgrades.get(0);
    }

}
