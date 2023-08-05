package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;

public class EnemyBinding implements ShaderBinding{
    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getTexture("enemy");

    }
}
