package cz.empatix.upgrades.service;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.TalentPath;
import cz.empatix.upgrades.domain.TreeType;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Stream;

public class UpgradeTree {
    private final CommonImage treeIcon;
    private final List<PlayerUpgrade> upgrades;
    private final List<TalentPath> parentConnectors;

    public UpgradeTree(final List<PlayerUpgrade> upgrades, final TreeType type) {
        this.upgrades = upgrades;
        this.parentConnectors = upgrades.stream()
                .skip(1)
                .limit(upgrades.size() - 1)
                .map(upgrade -> {
                    final TalentPath path = new TalentPath(type, upgrade.isLocked());
                    return path;
                })
                .toList();

        final String texture = switch (type) {
            case OFFENSIVE -> "ingame/offense_icon.tga";
            case DEFENSIVE -> "ingame/defense_icon.tga";
            case UTILITY -> "ingame/utility_icon.tga";
        };
        treeIcon = new CommonImage(texture, new Vector2f(), 4f, 3, RenderScene.OVERLAYS);

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
}
