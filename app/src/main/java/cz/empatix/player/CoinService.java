package cz.empatix.player;
import cz.empatix.core.Viewport;

import com.google.common.base.Preconditions;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector3i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class CoinService {
    private final static String DEFAULT_COIN_TEXTURE = "ingame/banecoin.tga";
    private final static Vector3i DEFAULT_COLOR_TEXT = new Vector3i(255, 216, 0);
    private final static int COIN_HUD_OFFSET = 10;
    private final static int TEXT_OFFSET = 15;
    private static final int IMAGE_SCALE = 2;
    private final static Logger LOGGER = LoggerFactory.getLogger(CoinService.class);
    private final CommonImage coinHud;
    private final CommonText coinText;
    private int coins = 0;

    public CoinService(final String coinTexture, final Vector3i colorText){
        final TextureAtlas.Entry coinTxt = AssetsService.getTexture(coinTexture);
        coinHud = new CommonImage(coinTxt, new Vector2f(COIN_HUD_OFFSET, Viewport.TARGET_HEIGHT - COIN_HUD_OFFSET - coinTxt.height() * IMAGE_SCALE), IMAGE_SCALE);
        coinText = new CommonText(String.valueOf(coins), "PixelFJVerdana12pt.ttf", new Vector2f(COIN_HUD_OFFSET + coinTxt.width() * IMAGE_SCALE + TEXT_OFFSET, 0f), coinHud.getZIndex());

        coinText.setCenterY(Viewport.TARGET_HEIGHT - COIN_HUD_OFFSET, Viewport.TARGET_HEIGHT - COIN_HUD_OFFSET - coinTxt.height() * IMAGE_SCALE);
        coinText.setColorRGB(colorText.x(), colorText.y(), colorText.z());
    }
    public CoinService(){
        this(DEFAULT_COIN_TEXTURE, DEFAULT_COLOR_TEXT);
    }
    public void addCoins(final int coins) {
        LOGGER.info("Adding {} coins", coins);
        Preconditions.checkArgument(coins > 0);
        this.coins += coins;
        updateHud();
    }
    private void updateHud(){
        coinText.setText(String.valueOf(coins));
    }

    public void removeCoins(final int coins) {
        LOGGER.info("Removing {} coins", coins);
        Preconditions.checkArgument(coins > 0);
        this.coins -= coins;
        updateHud();
    }
    public void collectSubRenderables(RenderableState state) {
        state.add(coinHud);
        coinText.collectSubRenderables(state);
    }

    public boolean hasEnoughCoins(final int requiredCoins) {
        return requiredCoins <= coins;
    }
}
