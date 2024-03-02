package cz.empatix.artefacts.domain;

import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.particles.service.GazewindService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;

import java.util.List;

public class BerserkPot implements Artefact{
    private static final String TEXTURE_PATH = "ingame/artefacts/berserkpot.tga";
    private final static int ARTEFACT_SCALE = 2;
    private final static int ACTIVATION_TIME = 15000;
    private final static int GAZE_SPEED_PRELIMIT = 350;
    private final static int GAZE_OFFSET = 10;
    private final CommonImage artefactImage;
    private final Player player;
    private BerserkMoveForceEventHook berserkMoveForceEventHook;
    private final GazewindService gazewindService;
    private long activationTime;

    public BerserkPot(final Player player) {
        this.player = player;
        artefactImage = new CommonImage(TEXTURE_PATH, new Vector2f(), ARTEFACT_SCALE);
        artefactImage.setPosition(ArtefactsService.HUD_SLOT_CENTER_X - artefactImage.getWidth() / 2f, ArtefactsService.HUD_SLOT_CENTER_Y - artefactImage.getHeight() / 2f);
        gazewindService = new GazewindService("ingame/artefacts/berserkpot-particle.tga");
    }

    @Override
    public void activate() {
        berserkMoveForceEventHook = new BerserkMoveForceEventHook();
        player.addHook(berserkMoveForceEventHook);
        activationTime = System.currentTimeMillis();
    }

    @Override
    public void update(double dt) {
        gazewindService.update((float) dt);
    }

    @Override
    public void fixedUpdate(double dt) {
        if (System.currentTimeMillis() - activationTime > ACTIVATION_TIME) {
            player.removeHook(berserkMoveForceEventHook);
            berserkMoveForceEventHook = null;
            gazewindService.clear();
        }
        if (berserkMoveForceEventHook != null) {
            final Vector2f velocity = player.getVelocity();
            if (Math.abs(velocity.x()) >= GAZE_SPEED_PRELIMIT || Math.abs(velocity.y()) >= GAZE_SPEED_PRELIMIT) {
                final Vector2f nextGazePosition = player.getPossibleGazePosition(750);
                gazewindService.spawn(nextGazePosition.x(), nextGazePosition.y());
            }
        }

    }

    @Override
    public List<Renderable> getRenderables() {
        return gazewindService.toRender();
    }

    @Override
    public List<Renderable> hudToRender() {
        return List.of(artefactImage);
    }

    @Override
    public String getTexturePath() {
        return TEXTURE_PATH;
    }
}
