package cz.empatix.player;

import com.google.common.base.Preconditions;
import com.google.common.collect.Lists;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.hud.service.Image;
import cz.empatix.hud.service.Text;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class CoinService {
    private final static int COIN_HUD_OFFSET = 25;
    private final static Logger LOGGER = LoggerFactory.getLogger(CoinService.class);
    private final Image coinHud;
    private final Text coinText;
    private int coins = 0;

    public CoinService(){
        final TextureAtlas.Entry coinTxt = AssetsService.getTexture("ingame/banecoin.tga");
        coinHud = new Image(coinTxt, new Vector2f(COIN_HUD_OFFSET, WindowHandler.TARGET_HEIGHT - COIN_HUD_OFFSET - coinTxt.height()), 1f);
        coinText = new Text(String.valueOf(coins), "PixelFJVerdana12pt.ttf", new Vector2f(COIN_HUD_OFFSET + coinTxt.width() + 15, 0f), coinHud.getZIndex());

        coinText.setCenterY(WindowHandler.TARGET_HEIGHT - COIN_HUD_OFFSET, WindowHandler.TARGET_HEIGHT - COIN_HUD_OFFSET - coinTxt.height());
        coinText.setColorRGB(255, 216, 0);
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
    public List<Renderable> toRender(){
        return Stream.of(List.of(coinHud), coinText.getSubRenderables())
                .flatMap(List::stream)
                .collect(Collectors.toList());
    }

    public boolean hasEnoughCoins(final int requiredCoins) {
        return requiredCoins <= coins;
    }
}
