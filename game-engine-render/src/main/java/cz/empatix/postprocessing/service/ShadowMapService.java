package cz.empatix.postprocessing.service;

import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.TextureArrayFramebufferAttachment;
import cz.empatix.render.service.Camera;

import java.awt.*;

public class ShadowMapService {
    private final Framebuffer shadowMapFBO;

    public ShadowMapService(){
        shadowMapFBO = new Framebuffer();
        shadowMapFBO.attach(new TextureArrayFramebufferAttachment(4, 255));

        Camera.getInstance().addObserver(shadowMapFBO);
    }

    public void renderShadowMap(){
        shadowMapFBO.bind();
        shadowMapFBO.flush();

        shadowMapFBO.unbind();
    }
}
