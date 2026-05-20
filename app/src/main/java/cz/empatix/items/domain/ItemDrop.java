package cz.empatix.items.domain;

import cz.empatix.audio.service.AudioService;
import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.interaction.domain.Interactable;
import cz.empatix.render.attributes.Drawable;

public interface ItemDrop extends Drawable, Interactable, Targetable {
     AudioService AUDIO_SERVICE = AudioService.getInstance();
    void update(final float dt);
    boolean canRemove();
    boolean isPickedUp();
    default DropSource getSource(){
        return DropSource.NORMAL;
    }
    void setPosition(final float x, final float y);
    void setDespawnable(final boolean despawn);
    int getWidth();
    int getHeight();
    void destroy();
}
