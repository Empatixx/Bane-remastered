package cz.empatix.artefacts.service;

import cz.empatix.artefacts.domain.*;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.utils.PositionUtils;
import cz.empatix.guns.domain.Gun;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.items.domain.MovableItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class ArtefactsService {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final static float HUD_SLOT_SCALE = 1.5f;
    public final static int HUD_SLOT_CENTER_X;
    public final static int HUD_SLOT_CENTER_Y;
    private final static Vector2f HUD_SLOT_POSITION = new Vector2f(585, 450);
    private final List<Artefact> allArtefacts;
    private Artefact currentArtefact;
    private final CommonImage commonImage;
    private boolean shouldActivate = false;
    private final ItemService itemService;
    private final Player player;

    public ArtefactsService(final Player player,
                            final ItemService itemService,
                            final MinimapService minimapService) {
        this.player = player;
        this.itemService = itemService;
        this.allArtefacts = List.of(
                new RingOfDeath(player),
                new ArmorRefill(player),
                new ShieldHorn(player),
                new BerserkPot(player),
                new RevealMap(minimapService)
        );
        currentArtefact = allArtefacts.get(4);
        commonImage = new CommonImage("ingame/artefacthud.tga", new Vector2f(), HUD_SLOT_SCALE,0);
        commonImage.setPosition(HUD_SLOT_POSITION.x(), HUD_SLOT_POSITION.y());
    }

    static {
        final TextureAtlas.Entry hudSlotTexture = AssetsService.getTexture("ingame/artefacthud.tga");

        HUD_SLOT_CENTER_X = PositionUtils.getCenterXInRange((int) (HUD_SLOT_POSITION.x()), (int) (HUD_SLOT_POSITION.x() + hudSlotTexture.width() * HUD_SLOT_SCALE));
        HUD_SLOT_CENTER_Y = PositionUtils.getCenterYInRange((int) (HUD_SLOT_POSITION.y()), (int) (HUD_SLOT_POSITION.y() + hudSlotTexture.height() * HUD_SLOT_SCALE));
    }

    public void update(final double dt) {
        allArtefacts.forEach(artefact -> artefact.update(dt));

    }

    public void fixedUpdate(final double dt) {
        if (shouldActivate) {
            if (currentArtefact != null) {
                currentArtefact.activate();
                currentArtefact = null;
            }
            shouldActivate = false;
        }
        allArtefacts.forEach(artefact -> artefact.fixedUpdate(dt));
    }

    public void keyPressed(final int keyCode) {
        if (keyCode == GLFW.GLFW_KEY_F) {
            shouldActivate = true;
        }
    }

    public List<Renderable> toRender() {
        final ArrayList<Renderable> renderables = new ArrayList<>(allArtefacts.size() + 1);
        if (currentArtefact != null) {
            renderables.add(commonImage);
            renderables.addAll(currentArtefact.hudToRender());

        }
        renderables.addAll(allArtefacts.stream()
                .flatMap(artefact -> artefact.getRenderables().stream())
                .toList());
        return renderables;
    }

    public Artefact getUnusedRandomArtefact() {
        Artefact artefact = allArtefacts.get(RANDOM.nextInt(allArtefacts.size()));
        while (artefact == currentArtefact) {
            artefact = allArtefacts.get(RANDOM.nextInt(allArtefacts.size()));
        }
        return artefact;
    }

    public void addArtefact(final Artefact artefact) {
        if (currentArtefact != null) {
            dropCurrentArtefact();
        }
        currentArtefact = artefact;
    }
    public void dropCurrentArtefact() {
        final ItemFactory itemFactory = itemService.getItemFactory();

        final MovableItemDrop itemDrop = itemFactory.createMovableArtefactDrop(currentArtefact, this::addArtefact);
        final Vector2f playerPosition = player.getPosition();
        final int playerWidth = player.getWidth();
        final int playerHeight = player.getHeight();
        final float x = playerPosition.x() + playerWidth / 2f - itemDrop.getWidth() / 2f;
        final float y = playerPosition.y() + playerHeight / 2f - itemDrop.getHeight() / 2f;
        itemDrop.setPosition(x, y);

        itemService.add(itemDrop);
    }
}
