package cz.empatix.keyboard.service;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.keyboard.domain.KeyboardImage;
import cz.empatix.keyboard.domain.KeyboardImageWithText;
import cz.empatix.keyboard.domain.KeyboardType;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.service.StartRoom;
import org.joml.Vector2f;
import org.joml.Vector3i;

import java.util.ArrayList;
import java.util.List;


public class StartRoomKeyboardService {
    private final List<KeyboardImageWithText> pairs;
    private final List<KeyboardImage> keyboardImages;
    private final AlternativeViewText movementDesc;

    public StartRoomKeyboardService(final StartRoom room) {
        final KeyboardImage dropGunKey = new KeyboardImage(KeyboardType.Q);
        final KeyboardImage interactKey = new KeyboardImage(KeyboardType.E);
        final KeyboardImage useArtefactKey = new KeyboardImage(KeyboardType.F);
        final KeyboardImage useSlot1Key = new KeyboardImage(KeyboardType.NUMBER_ONE);
        final KeyboardImage useSlot2Key = new KeyboardImage(KeyboardType.NUMBER_TWO);

        final AlternativeViewText dropGunDesc = new AlternativeViewText("Drop gun", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getInstance().getProjectionMatrix(), RenderScene.SCENE);
        final AlternativeViewText interactDesc = new AlternativeViewText("Interact", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getInstance().getProjectionMatrix(), RenderScene.SCENE);
        final AlternativeViewText useArtefactDesc = new AlternativeViewText("Use artefact", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getInstance().getProjectionMatrix(), RenderScene.SCENE);
        final AlternativeViewText useSlot1Desc = new AlternativeViewText("Switch slot 1", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getInstance().getProjectionMatrix(), RenderScene.SCENE);
        final AlternativeViewText useSlot2Desc = new AlternativeViewText("Switch slot 2", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getInstance().getProjectionMatrix(), RenderScene.SCENE);

        final KeyboardImageWithText dropGunPair = KeyboardImageWithText.of(dropGunKey, dropGunDesc);
        final KeyboardImageWithText interactionPair = KeyboardImageWithText.of(interactKey, interactDesc);
        final KeyboardImageWithText useArtefactPair = KeyboardImageWithText.of(useArtefactKey, useArtefactDesc);
        final KeyboardImageWithText useSlot1Pair = KeyboardImageWithText.of(useSlot1Key, useSlot1Desc);
        final KeyboardImageWithText useSlot2Pair = KeyboardImageWithText.of(useSlot2Key, useSlot2Desc);

        final float xRoom = room.getX() + room.getWidth() / 2f;
        final float yRoom = room.getY();

        pairs = List.of(dropGunPair, interactionPair, useArtefactPair, useSlot1Pair, useSlot2Pair);
        for (int rows = 0; rows < pairs.size(); rows++) {
            pairs.get(rows).setPosition(xRoom, yRoom + 300 + rows * 50);
        }

        final KeyboardImage moveUpKey = new KeyboardImage(KeyboardType.W);
        final KeyboardImage moveDownKey = new KeyboardImage(KeyboardType.S);
        final KeyboardImage moveLeftKey = new KeyboardImage(KeyboardType.A);
        final KeyboardImage moveRightKey = new KeyboardImage(KeyboardType.D);

        final AlternativeViewText movementText = new AlternativeViewText("Movement", "PixelFJVerdana12pt.ttf", new Vector2f(), 1, FontSize.FONTSIZE_22, Camera.getInstance().getProjectionMatrix(), RenderScene.SCENE);

        moveUpKey.setPosition(xRoom - 150, yRoom + 350);
        moveDownKey.setPosition(xRoom - 150, yRoom + 400);
        moveLeftKey.setPosition(xRoom - 200, yRoom + 400);
        moveRightKey.setPosition(xRoom - 100, yRoom + 400);

        movementText.setPosition(0, yRoom + 470);
        movementText.setCenterX(moveLeftKey.getPosition().x(), moveRightKey.getPosition().x() + moveRightKey.getWidth());

        this.movementDesc = movementText;
        this.keyboardImages = List.of(moveDownKey, moveUpKey, moveLeftKey, moveRightKey);

    }


    public List<Renderable> toRender() {
        final List<Renderable> renderables = new ArrayList<>(pairs.size() + keyboardImages.size() + 1);
        renderables.addAll(movementDesc.getSubRenderables());
        renderables.addAll(pairs);
        renderables.addAll(keyboardImages);
        return renderables;
    }

    public void update(double dt) {
        pairs.forEach(k -> k.update(dt));
        keyboardImages.forEach(k -> k.update(dt));
    }
}
