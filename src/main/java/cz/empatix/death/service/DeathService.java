package cz.empatix.death.service;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.gamestate.service.GameStateController;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.postprocessing.domain.Fade;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;

public class DeathService {
    private final static int SKULL_INDEX = 4;
    private final static float SKULL_TRANSITION_TIME = 2.5f;
    private final static int SKULL_START_POSITION_Y = 200;
    private final static int SKULL_END_POSITION_Y = 50;
    private final static int SKULL_DIFFERENCE_Y = SKULL_START_POSITION_Y - SKULL_END_POSITION_Y;
    private final static int TOTAL_LABELS = 4;
    private final static int LABEL_POS_OFFSET = 50;
    private final static float LABEL_TRANSITION_TIME = .5f;
    private final static int LABEL_START_POSITION_Y = 250;
    private final static int LABEL_END_POSITION_Y = 200;
    private final static int LABEL_DIFFERENCE_Y = LABEL_START_POSITION_Y - LABEL_END_POSITION_Y;
    private static final int LABEL_TEXT_END_POSITION_Y = 225;
    private static final int LABEL_TEXT_START_POSITION_Y = LABEL_TEXT_END_POSITION_Y + LABEL_DIFFERENCE_Y;
    private final static float REWARD_TRANSITION_TIME = 0.5f;
    private final List<CommonImage> icons;
    private final List<CommonText> texts;
    private final List<CommonText> rewardTexts;
    private final CommonText continueText;
    private final CommonText gameOverText;
    private boolean active = false;
    private boolean continuePressed = false;
    private float accumulator = 0f;

    public DeathService() {

        final CommonImage accuracyIcon = new CommonImage("ingame/accuracy_icon.tga", new Vector2f(250, LABEL_START_POSITION_Y), 1f, 2, RenderScene.OVERLAYS);
        final CommonImage killsIcon = new CommonImage("ingame/kills_icon.tga", new Vector2f(250, LABEL_START_POSITION_Y + LABEL_POS_OFFSET), 1f, 2, RenderScene.OVERLAYS);
        final CommonImage lootIcon = new CommonImage("ingame/loot_icon.tga", new Vector2f(250, LABEL_START_POSITION_Y + 2 * LABEL_POS_OFFSET), 1f, 2, RenderScene.OVERLAYS);
        final CommonImage floorIcon = new CommonImage("ingame/floor_icon.tga", new Vector2f(250, LABEL_START_POSITION_Y + 3 * LABEL_POS_OFFSET), 1f, 2, RenderScene.OVERLAYS);

        final CommonImage skullImage = new CommonImage("ingame/skull.tga", new Vector2f(429, SKULL_START_POSITION_Y), 6f, 2, RenderScene.OVERLAYS);

        icons = List.of(accuracyIcon, killsIcon, lootIcon, floorIcon, skullImage);

        final CommonText accuracyText = new CommonText("Accuracy: 100%", "PixelFJVerdana12pt.ttf", new Vector2f(300, LABEL_TEXT_START_POSITION_Y), 2, RenderScene.OVERLAYS);
        final CommonText killsText = new CommonText("Kills: 0", "PixelFJVerdana12pt.ttf", new Vector2f(300, LABEL_TEXT_START_POSITION_Y + LABEL_POS_OFFSET), 2, RenderScene.OVERLAYS);
        final CommonText lootText = new CommonText("Loot: 0", "PixelFJVerdana12pt.ttf", new Vector2f(300, LABEL_TEXT_START_POSITION_Y + 2 * LABEL_POS_OFFSET), 2, RenderScene.OVERLAYS);
        final CommonText floorText = new CommonText("Floor: 1", "PixelFJVerdana12pt.ttf", new Vector2f(300, LABEL_TEXT_START_POSITION_Y + 3 * LABEL_POS_OFFSET), 2, RenderScene.OVERLAYS);

        final CommonText accuracyCoins = new CommonText("+0", "PixelFJVerdana12pt.ttf", new Vector2f(650, LABEL_TEXT_END_POSITION_Y), 2, RenderScene.OVERLAYS);
        final CommonText killsCoins = new CommonText("+0", "PixelFJVerdana12pt.ttf", new Vector2f(650, LABEL_TEXT_END_POSITION_Y + LABEL_POS_OFFSET), 2, RenderScene.OVERLAYS);
        final CommonText lootCoins = new CommonText("+0", "PixelFJVerdana12pt.ttf", new Vector2f(650, LABEL_TEXT_END_POSITION_Y + 2 * LABEL_POS_OFFSET), 2, RenderScene.OVERLAYS);
        final CommonText floorCoins = new CommonText("+0", "PixelFJVerdana12pt.ttf", new Vector2f(650, LABEL_TEXT_END_POSITION_Y + 3 * LABEL_POS_OFFSET), 2, RenderScene.OVERLAYS);

        gameOverText = new CommonText("Game Over", "PixelFJVerdana12pt.ttf", new Vector2f(0, 175), 2, RenderScene.OVERLAYS, FontSize.FONTSIZE_24);
        gameOverText.setCenterX(0, WindowHandler.TARGET_WIDTH);

        continueText = new CommonText("Press any key to continue...", "PixelFJVerdana12pt.ttf", new Vector2f(700, 500), 2, RenderScene.OVERLAYS);

        accuracyCoins.setColorRGB(212, 168, 8);
        killsCoins.setColorRGB(212, 168, 8);
        lootCoins.setColorRGB(212, 168, 8);
        floorCoins.setColorRGB(212, 168, 8);

        texts = List.of(accuracyText, killsText, lootText, floorText);
        rewardTexts = List.of(accuracyCoins, killsCoins, lootCoins, floorCoins);

    }

    public void update(final float dt, final boolean isPlayerDead) {
        if (isPlayerDead && !active) {
            CursorService.setCursor(CursorType.DISABLED);
            Fade.setTween(0.2f);
            active = true;
        }
        if (!active) {
            return;
        }
        accumulator += dt;
        if (accumulator > 0f) {
            final CommonImage skullImage = icons.get(SKULL_INDEX);
            skullImage.setPosition(skullImage.getX(), SKULL_START_POSITION_Y - SKULL_DIFFERENCE_Y * Math.min(SKULL_TRANSITION_TIME, accumulator) / SKULL_TRANSITION_TIME);
        }
        for (int i = 0; i < TOTAL_LABELS; i++) {
            if (accumulator > i * LABEL_TRANSITION_TIME + SKULL_TRANSITION_TIME) {
                final CommonText text = texts.get(i);
                final CommonImage icon = icons.get(i);

                final float offsetAccumulator = accumulator - i * LABEL_TRANSITION_TIME - SKULL_TRANSITION_TIME;
                text.setPosition(text.getX(), LABEL_TEXT_START_POSITION_Y + i * LABEL_POS_OFFSET - LABEL_DIFFERENCE_Y * Math.min(LABEL_TRANSITION_TIME, offsetAccumulator) / LABEL_TRANSITION_TIME);
                icon.setPosition(icon.getX(), LABEL_START_POSITION_Y + i * LABEL_POS_OFFSET - LABEL_DIFFERENCE_Y * Math.min(LABEL_TRANSITION_TIME, offsetAccumulator) / LABEL_TRANSITION_TIME);
            } else {
                break;
            }
        }
        for (int i = 0; i < TOTAL_LABELS; i++) {
            if (accumulator > i * REWARD_TRANSITION_TIME + TOTAL_LABELS * LABEL_TRANSITION_TIME + SKULL_TRANSITION_TIME) {
                final CommonText text = rewardTexts.get(i);
                final float offsetAccumulator = accumulator - i * REWARD_TRANSITION_TIME - TOTAL_LABELS * LABEL_TRANSITION_TIME - SKULL_TRANSITION_TIME;
                final float delta = Math.min(REWARD_TRANSITION_TIME, offsetAccumulator) / REWARD_TRANSITION_TIME;
                text.setText("+" + (int) ((i * 100) * delta));
            } else {
                break;
            }
        }
        if (!continuePressed && Fade.getValue() > 1) {
            Fade.setTween(0);
        } else if (continuePressed && Fade.getValue() > 2) {
            GameStateController.getInstance().setCurrentState(GameStateType.MENU);
            Fade.setTween(-1);
        }
    }

    public List<Renderable> toRender() {
        if (!active || continuePressed) {
            return List.of();
        }
        final ArrayList<Renderable> renderables = new ArrayList<>(100);
        renderables.add(icons.get(SKULL_INDEX));
        if(accumulator > SKULL_TRANSITION_TIME) {
            renderables.addAll(gameOverText.getSubRenderables());
        }
        for (int i = 0; i < TOTAL_LABELS; i++) {
            if (accumulator > i * LABEL_TRANSITION_TIME + SKULL_TRANSITION_TIME) {
                renderables.addAll(texts.get(i).getSubRenderables());
                renderables.add(icons.get(i));
            } else {
                break;
            }
        }
        for (int i = 0; i < TOTAL_LABELS; i++) {
            if (accumulator > i * REWARD_TRANSITION_TIME + TOTAL_LABELS * LABEL_TRANSITION_TIME + SKULL_TRANSITION_TIME) {
                renderables.addAll(rewardTexts.get(i).getSubRenderables());
            } else {
                break;
            }
        }
        if (System.currentTimeMillis() / 200 % 2 == 0 && accumulator > TOTAL_LABELS * (REWARD_TRANSITION_TIME + LABEL_TRANSITION_TIME) + SKULL_TRANSITION_TIME) {
            renderables.addAll(continueText.getSubRenderables());
        }
        return List.copyOf(renderables);
    }

    public void keyPressed(final int key) {
        if (active && !continuePressed && accumulator > TOTAL_LABELS * (REWARD_TRANSITION_TIME + LABEL_TRANSITION_TIME) + SKULL_TRANSITION_TIME) {
            continuePressed = true;
            Fade.setTween(1);
        }
    }

    public boolean isActive() {
        return active;
    }
}
